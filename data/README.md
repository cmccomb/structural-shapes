# AISC catalog source

`aisc-v16.csv` contains the W and HSS numeric section data and designations
extracted from the American Institute of Steel Construction (AISC) Shapes
Database v16.0, August 2023, consistent with the *Steel Construction Manual*,
16th Edition, 1st Printing.

- [AISC database page](https://www.aisc.org/aisc/publications/steel-construction-manual/aisc-shapes-database-v160/)
- [Source workbook](https://cloud.aisc.org/biggie_bin/aisc-shapes-database-v160-2.xlsx)
- Retrieved: 2026-09-19
- Workbook SHA-256: `82d0ceb96a0d938ae1a6bd9637cb10a1e269225b5d668dce5b0bdc8d86013496`
- Sheet: `Database v16.0`, US customary columns A:CF (not the separately rounded metric columns).
- Coverage: 289 W, 126 square HSS, 399 rectangular HSS, and 189 round HSS sections.

The upstream workbook is the source of the tabulated facts, terminology, and
disclaimer. This project does not claim AISC affiliation or endorsement. The
extract does not reproduce the workbook's illustrations, layout, or explanatory
text. Catalog inclusion does not establish current commercial availability or
constitute a structural design/code-compliance check. No material grade, yield
strength, or ASTM A1085-specific catalog is inferred.

## Fields and units

The CSV retains source column names and source values; an empty cell means the
dimension does not apply. It is never interpreted as zero.

| Fields | Meaning | Source units |
| --- | --- | --- |
| Type, EDI_Std_Nomenclature, AISC_Manual_Label | Family and US designations | Text |
| W | Nominal weight per length (exposed as mass per length) | lb/ft |
| A | Cross-sectional area | in² |
| d, bf, tw, tf | W depth, flange width, web thickness, flange thickness | in |
| Ht, B | Rectangular/square HSS height and width | in |
| OD | Round HSS outside diameter | in |
| tnom, tdes | HSS nominal and design thickness | in |
| Ix, Iy | Centroidal moments about the horizontal/vertical axes | in⁴ |
| J | Saint-Venant torsional constant | in⁴ |

The Rust API converts these values to `uom` SI quantities without recomputing
the tabulated properties. Conversion uses 0.0254 m/in, 0.3048 m/ft, and
0.45359237 kg/lb. It does not imply additional source precision. `J` is distinct
from `Ix + Iy`; even round-section tabulated values may differ through rounding.
Idealized geometry uses `tdes` and omits fillets/rectangular corner radii.

## Reproduction

Normal development needs only Python 3's standard library and no network:

```sh
python3 scripts/generate_aisc.py
python3 scripts/generate_aisc.py --check
```

To reproduce the CSV directly from the source, download the linked workbook
and install `openpyxl` in a separate Python environment, then run:

```sh
python3 scripts/generate_aisc.py --workbook /path/to/aisc-v16.xlsx
python3 scripts/generate_aisc.py --workbook /path/to/aisc-v16.xlsx --check
```

The importer requires the recorded checksum. A new edition or corrected source
requires an explicit provenance update and review, not an automatic refresh.
The generator checks positive finite values, geometry constraints, unique names
and Rust identifiers, unambiguous aliases, and family counts before writing.
Cargo uses checked-in generated Rust; it does not run Python or download data.
