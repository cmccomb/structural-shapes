#!/usr/bin/env python3
"""Generate the Rust catalog from the checked-in AISC v16.0 numeric extract.

Normal generation/checking uses only the Python standard library. Importing the
pinned upstream workbook with --workbook additionally requires openpyxl.
"""

import argparse
from collections import Counter
import csv
from decimal import Decimal
from fractions import Fraction
import hashlib
import io
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
CSV_PATH = ROOT / "data/aisc-v16.csv"
RUST_PATH = ROOT / "src/aisc/generated.rs"
WORKBOOK_SHA256 = "82d0ceb96a0d938ae1a6bd9637cb10a1e269225b5d668dce5b0bdc8d86013496"
FIELDS = "Type EDI_Std_Nomenclature AISC_Manual_Label W A d Ht OD bf B tw tf tnom tdes Ix Iy J b t Sx Sy Zx Zy rx ry Cw C eo rts ho Iw Iz tan(α)".split()
# Missing values are expected only outside these source families. Reject a
# truncated extract instead of silently converting missing required data to None.
OPTIONAL_FAMILIES = {
    "Cw": {"W", "M", "S", "HP", "C", "MC", "L", "WT", "MT", "ST"},
    "C": {"HSS"}, "eo": {"C", "MC"},
    "rts": {"W", "M", "S", "HP", "C", "MC"},
    "ho": {"W", "M", "S", "HP", "C", "MC"},
    "Iw": {"L"}, "Iz": {"L"}, "tan(α)": {"L"},
}
FAMILIES = {
    "W": "WideFlange", "M": "MiscellaneousBeam", "S": "AmericanStandardBeam",
    "HP": "BearingPile", "C": "AmericanStandardChannel", "MC": "MiscellaneousChannel",
    "L": "Angle", "WT": "WideFlangeTee", "MT": "MiscellaneousTee",
    "ST": "AmericanStandardTee", "2L": "DoubleAngle", "PIPE": "Pipe",
}
COUNTS = {
    "WideFlange": 289, "MiscellaneousBeam": 16, "AmericanStandardBeam": 28,
    "BearingPile": 22, "AmericanStandardChannel": 32, "MiscellaneousChannel": 40,
    "Angle": 137, "WideFlangeTee": 289, "MiscellaneousTee": 14,
    "AmericanStandardTee": 28, "DoubleAngle": 639, "Pipe": 51,
    "SquareHollow": 126, "RectangularHollow": 399, "RoundHollow": 189,
}


def workbook_csv(path):
    """Read only the pinned workbook's US customary columns (A:CF)."""
    if hashlib.sha256(path.read_bytes()).hexdigest() != WORKBOOK_SHA256:
        raise ValueError("Workbook checksum differs from the pinned v16.0 source")
    from openpyxl import load_workbook

    workbook = load_workbook(path, read_only=True, data_only=True)
    rows = iter(workbook["Database v16.0"].values)
    header = next(rows)[:84]
    output = io.StringIO(newline="")
    writer = csv.DictWriter(output, fieldnames=FIELDS, lineterminator="\n")
    writer.writeheader()
    for values in rows:
        if values[0] is None:
            continue
        source = dict(zip(header, values[:84]))
        writer.writerow({key: "" if source[key] == "–" else source[key] for key in FIELDS})
    workbook.close()
    return output.getvalue()


def number(row, name):
    """Validate finite positive source data and produce a Rust f64 literal."""
    value = Decimal(row[name])
    if not value.is_finite() or value <= 0:
        raise ValueError(f"Invalid {name} for {row['AISC_Manual_Label']}")
    literal = format(value, "f")
    return literal if "." in literal else literal + ".0"


def fraction(text):
    """Interpret an AISC fraction or mixed number in inches exactly."""
    return sum((Fraction(part) for part in text.split("-")), Fraction(0))


def optional_number(row, name):
    """Preserve family-specific missing values and validate applicable values."""
    if row["Type"] in OPTIONAL_FAMILIES[name]:
        return f"Some({number(row, name)})"
    if row[name]:
        raise ValueError(f"Unexpected {name} for {row['AISC_Manual_Label']}")
    return "None"


def double_angle_gap(row):
    """Decode spacing and verify orientation against each double-angle record."""
    label = row["AISC_Manual_Label"]
    match = re.fullmatch(r"2L([0-9./-]+)X([0-9./-]+)X([0-9./-]+)(?:X([0-9./-]+))?(LLBB|SLBB)?", label)
    if not match:
        raise ValueError(f"Invalid double-angle designation: {label}")
    long_leg, short_leg, thickness, gap, orientation = match.groups()
    long_leg, short_leg = fraction(long_leg), fraction(short_leg)
    if long_leg < short_leg or (long_leg != short_leg and orientation is None):
        raise ValueError(f"Missing or inconsistent double-angle orientation: {label}")
    expected = (short_leg, long_leg) if orientation == "SLBB" else (long_leg, short_leg)
    if tuple(Fraction(row[key]) for key in ("d", "b")) != expected:
        raise ValueError(f"Double-angle dimensions disagree with orientation: {label}")
    # t is a rounded source value, so do not replace it with the label fraction.
    if abs(Fraction(row["t"]) - fraction(thickness)) > Fraction(1, 100):
        raise ValueError(f"Double-angle thickness disagrees with label: {label}")
    value = fraction(gap) if gap else Fraction(0)
    if value < 0:
        raise ValueError(f"Negative double-angle gap: {label}")
    return str(float(value))


def geometry(row):
    """Map source dimensions to explicit geometry types, then validate them."""
    typ = row["Type"]
    if typ in ("W", "M", "S", "HP", "C", "MC", "WT", "MT", "ST"):
        family = FAMILIES[typ]
        kind = "Channel" if typ in ("C", "MC") else "Tee" if typ in ("WT", "MT", "ST") else "IBeam"
        dims = [number(row, key) for key in ("d", "bf", "tw", "tf")]
        height, width, web, flange = map(Decimal, dims)
        valid = web < width and flange * (1 if kind == "Tee" else 2) < height
    elif typ == "L":
        family, kind = "Angle", "Angle"
        # In the AISC single-angle orientation the longer leg (b) is vertical.
        dims = [number(row, key) for key in ("b", "d", "t")]
        height, width, thickness = map(Decimal, dims)
        valid = thickness < min(height, width)
    elif typ == "2L":
        family, kind = "DoubleAngle", "DoubleAngle"
        dims = [number(row, key) for key in ("d", "b", "t")] + [double_angle_gap(row)]
        height, width, thickness, _ = map(Decimal, dims)
        valid = thickness < min(height, width)
    elif typ == "PIPE" or (typ == "HSS" and row["OD"]):
        family = "Pipe" if typ == "PIPE" else "RoundHollow"
        kind = "HollowRound"
        dims = [number(row, key) for key in ("OD", "tnom", "tdes")]
        diameter, nominal, design = map(Decimal, dims)
        valid = design <= nominal and 2 * nominal < diameter
    elif typ == "HSS":
        family = "SquareHollow" if Decimal(row["Ht"]) == Decimal(row["B"]) else "RectangularHollow"
        kind = "HollowRectangle"
        dims = [number(row, key) for key in ("Ht", "B", "tnom", "tdes")]
        height, width, nominal, design = map(Decimal, dims)
        valid = design <= nominal and 2 * nominal < min(height, width)
    else:
        raise ValueError(f"Unsupported family: {typ}")
    if not valid:
        raise ValueError(f"Invalid geometry: {row['AISC_Manual_Label']}")
    return family, f'RawDimensions::{kind}([{", ".join(dims)}])'


def generate(source):
    """Validate names/geometry and emit one catalog entry per source row."""
    reader = csv.DictReader(io.StringIO(source))
    if reader.fieldnames != FIELDS:
        raise ValueError("Unexpected CSV schema")
    lines = [
        "// Generated by scripts/generate_aisc.py; do not edit.",
        "// Source: data/aisc-v16.csv; provenance and units: data/README.md.",
        "// Geometry: RawDimensions. Properties: [W, A, Ix, Iy, Sx, Sy, Zx, Zy, rx, ry], J.",
        "// Optional stability: [Cw, C, eo, rts, ho]; angle principal data: [Iw, Iz, tan(alpha)].",
        "catalog! {",
    ]
    identifiers, aliases, counts = set(), {}, Counter()
    for row in reader:
        label, edi = row["AISC_Manual_Label"], row["EDI_Std_Nomenclature"]
        identifier = re.sub(r"[./-]", "_", label)
        if identifier.startswith("2L"):
            identifier = "DoubleAngle" + identifier[2:]
        if not re.fullmatch(r"[A-Za-z][A-Za-z0-9_]*", identifier) or identifier in identifiers:
            raise ValueError(f"Invalid or duplicate Rust identifier: {identifier}")
        identifiers.add(identifier)
        for alias in (label, edi):
            if not re.fullmatch(r"[A-Za-z0-9./-]+", alias):
                raise ValueError(f"Invalid designation: {alias}")
            alias = alias.upper()
            if alias in aliases and aliases[alias] != identifier:
                raise ValueError(f"Ambiguous designation: {alias}")
            aliases[alias] = identifier
        family, dimensions = geometry(row)
        properties = [number(row, key) for key in ["W", "A", "Ix", "Iy", "Sx", "Sy", "Zx", "Zy", "rx", "ry"]]
        if row["Type"] == "2L":
            if row["J"]:
                raise ValueError(f"Unexpected tabulated double-angle J: {label}")
            torsion = "None"
        else:
            torsion = f'Some({number(row, "J")})'
        stability = [optional_number(row, key) for key in ["Cw", "C", "eo", "rts", "ho"]]
        for key in ["Iw", "Iz", "tan(α)"]:
            optional_number(row, key)
        principal = "None"
        if row["Type"] == "L":
            if Decimal(row["Iw"]) <= Decimal(row["Iz"]):
                raise ValueError(f"Invalid principal moments: {label}")
            principal = f'Some([{", ".join(number(row, key) for key in ["Iw", "Iz", "tan(α)"])}])'
        counts[family] += 1
        lines.append(f'    {identifier} => ("{label}", "{edi}", {family}, {dimensions}, [{", ".join(properties)}], {torsion}, [{", ".join(stability)}], {principal});')
    if counts != COUNTS:
        raise ValueError(f"Unexpected family counts: {dict(counts)}")
    return "\n".join(lines + ["}", ""])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Fail if checked-in data/code differ")
    parser.add_argument("--workbook", type=Path, help="Re-extract the pinned upstream XLSX")
    args = parser.parse_args()
    source = workbook_csv(args.workbook) if args.workbook else CSV_PATH.read_text()
    generated = generate(source)
    if args.check:
        if CSV_PATH.read_text() != source or RUST_PATH.read_text() != generated:
            parser.exit(1, "AISC catalog is out of date; run scripts/generate_aisc.py\n")
        print(f"AISC catalog verified: {sum(COUNTS.values())} sections")
    else:
        CSV_PATH.parent.mkdir(parents=True, exist_ok=True)
        RUST_PATH.parent.mkdir(parents=True, exist_ok=True)
        CSV_PATH.write_text(source)
        RUST_PATH.write_text(generated)
        print(f"Generated AISC catalog: {sum(COUNTS.values())} sections")


if __name__ == "__main__":
    main()
