# AISC catalog source

`aisc-v16.csv` contains the numeric section data and designations for all 2,299 entries
extracted from the American Institute of Steel Construction (AISC) Shapes
Database v16.0, August 2023, consistent with the *Steel Construction Manual*,
16th Edition, 1st Printing.

- [AISC database page](https://www.aisc.org/aisc/publications/steel-construction-manual/aisc-shapes-database-v160/)
- [Source workbook](https://cloud.aisc.org/biggie_bin/aisc-shapes-database-v160-2.xlsx)
- Retrieved: 2026-09-19
- Workbook SHA-256: `82d0ceb96a0d938ae1a6bd9637cb10a1e269225b5d668dce5b0bdc8d86013496`
- Sheet: `Database v16.0`, US customary columns A:CF (not the separately rounded metric columns).
- Coverage: all 13 source type codes (15 API families after splitting HSS by geometry).

| Source family | Entries |
| --- | ---: |
| W / M / S / HP | 289 / 16 / 28 / 22 |
| C / MC | 32 / 40 |
| L | 137 |
| WT / MT / ST | 289 / 14 / 28 |
| 2L | 639 |
| HSS square / rectangular / round | 126 / 399 / 189 |
| PIPE | 51 |

The upstream workbook is the source of the tabulated facts, terminology, and
disclaimer. This project does not claim AISC affiliation or endorsement. The
extract does not reproduce the workbook's illustrations, layout, or explanatory
text. Catalog inclusion does not establish current commercial availability or
constitute a structural design/code-compliance check. No material grade, yield
strength, or ASTM A1085-specific catalog is inferred.

## Fields and units

The CSV retains source column names and source values; an empty cell means the
dimension does not apply or a property is not supplied. It is never interpreted
as zero. In particular, all 639 double-angle entries have no tabulated `J`.

| Fields | Meaning | Source units |
| --- | --- | --- |
| Type, EDI_Std_Nomenclature, AISC_Manual_Label | Family and US designations | Text |
| W | Nominal weight per length (exposed as mass per length) | lb/ft |
| A | Cross-sectional area | in² |
| d, bf, tw, tf | Beam/channel/tee depth, flange width, web or stem thickness, flange thickness | in |
| b, d, t | Single-angle long leg, short leg, and thickness; double-angle outstanding leg, back-to-back leg, and thickness | in |
| Ht, B | Rectangular/square HSS height and width | in |
| OD | Round HSS outside diameter | in |
| tnom, tdes | HSS/pipe nominal and design thickness | in |
| Ix, Iy | Centroidal moments about the horizontal/vertical axes | in⁴ |
| J | Saint-Venant torsional constant, absent for 2L | in⁴ |
| Sx, Sy | Elastic section moduli | in³ |
| Zx, Zy | Plastic section moduli | in³ |
| rx, ry | Radii of gyration | in |
| Cw | Warping constant, where tabulated | in⁶ |
| C | HSS torsional section modulus; distinct from J | in³ |
| eo | Channel distance from AISC-designated edge to shear center | in |
| rts | Effective radius of gyration, where tabulated | in |
| ho | Distance between flange centroids, where tabulated | in |
| Iw, Iz | Single-angle major/minor principal second moments | in⁴ |
| tan(α) | Tangent of the angle between y and minor principal z for single angles | Dimensionless |

The Rust API converts these values to `uom` SI quantities without recomputing
the tabulated properties. Conversion uses 0.0254 m/in, 0.3048 m/ft, and
0.45359237 kg/lb. It does not imply additional source precision. `J` is distinct
from `Ix + Iy`; even round-section tabulated values may differ through rounding.
Idealized geometry uses `tdes` for HSS/pipe and omits fillets, toe/corner radii,
and flange taper. The generator retains the source values of `b` for HSS too,
where it denotes flat wall width; that field is not used in the idealization.

Single angles use the AISC orientation with the long leg (`b`) vertical and short
leg (`d`) horizontal. For double angles, the source's `d` and `b` reflect the
designated arrangement: `d` is the back-to-back leg and `b` the outward leg.
The generator checks these dimensions against LLBB/SLBB (long/short legs
back-to-back). Equal-leg pairs need no orientation suffix. Clear spacing is
decoded from the optional fourth size in the label, e.g. `X3/8`; mixed numbers
such as `X1-1/2` are supported and an omitted spacing is zero.

Double-angle `J` remains `None` in the Rust API. No stiffness or connection
assumption is made to synthesize a torsional constant. Angle x/y moments are
centroidal geometric-axis properties, not principal moments.

All 2,299 entries supply Sx, Sy, Zx, Zy, rx, and ry. Optional coverage is 895
entries for Cw, 714 for HSS C, 72 for channel eo, and 427 each for rts and ho.
All 137 single angles supply Iw, Iz, and tan(α). The generator checks these
family-specific availability rules, including rejecting a missing required cell.

Ixy is not a source column. The API derives it for single angles as
`-(Iw - Iz) * tan(α) / (1 + tan(α)^2)`, with `Ixy = integral(x*y dA)` and the
same left/up and bottom/right leg orientation as `idealized_shape()`. Other
families have zero centroidal Ixy by symmetry. This calculation uses published
principal properties, not sharp-corner geometry. Independently rounded Ix/Iy and
principal data need not form an exactly consistent tensor; calculated principal
moments may therefore differ slightly from tabulated Iw/Iz. Tabulated S, Z, r,
J, Cw, C, eo, rts, and ho are preserved without recomputation.

## Reproduction

Normal development needs only Python 3's standard library and no network:

```sh
python3 scripts/generate_aisc.py
python3 scripts/generate_aisc.py --check
python3 -m unittest discover -s scripts -p 'test_*.py'
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
