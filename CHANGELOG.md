# Changelog

## 0.3.0

### Added

- Optional `aisc` feature with all 2,299 sections from the AISC Shapes Database
  v16.0: W, M, S, HP, C, MC, L, WT, MT, ST, double angles, HSS, and pipe.
  Typed section identifiers support designation parsing, enumeration, and selection.
- Unit-aware catalog dimensions and properties: area, Ix/Iy, Sx/Sy, Zx/Zy,
  rx/ry, mass per length, and optional J, Cw, HSS C, channel eo, rts, and ho.
  Missing source values remain `None`. Single-angle Ixy is derived from rounded
  published principal-axis data with an explicit orientation and sign convention.
- Parametric channels, tees, angles, and spaced back-to-back double angles,
  available without the catalog feature.
- Signed products of inertia, centroidal area moments, axis rotations, principal
  moments and axes, and custom-shape radii of gyration.
- Exact, position-independent torsional constants for circular rods and concentric
  pipes. Other custom shapes return `None`; `polar_moi()` remains Ix + Iy.
- Checked composite-centroid calculation for undefined or nonfinite centroids.
- Reproducible catalog generation from a checksum-pinned source, source notes,
  and a standard-section selection example.

### Fixed

- I-beam x-axis inertia dispatch, the rectangle y-axis inertia formula, and
  swapped x/y parallel-axis translation terms. Numerical results for affected
  custom shapes and composites differ from 0.2.3.
- Circular-shaft example now evaluates stress at the outer surface and uses
  the explicit torsional constant.

### Compatibility

- The default feature set remains empty; enable `aisc` for the catalog.
- Existing constructors remain available and continue to assume valid dimensions.
- Catalog values and sharp-corner idealized geometry remain distinct. Custom
  composites use signed sums, not Boolean geometry operations.
