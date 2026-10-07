# MPBRI figure reproducibility in Rust

This repository contains the Rust implementation used to reproduce the **intrinsic theoretical figures** for the MPBRI manuscript, *Massively Parallel Pseudo-Marginal Inference for Fixed-Data Posterior-Predictive Policy Optimization*.

The repository is deliberately small and dependency-free. The plotting program uses only the Rust standard library and writes vector SVG files directly. The `figures` directory also contains the corresponding PDF figures used in the manuscript for convenient comparison.

## What is reproduced

The code regenerates the four figures appearing in the manuscript's **Intrinsic scaling laws** section. These are theoretical plots, not baseline experiments or empirical learning curves.

| Output | Mathematical quantity |
| --- | --- |
| `figures/relative_noise.svg` | Single-group relative-variance bound from `Var(Lhat)/L_c^2 <= (1-L_c)(L_c-c)/(B L_c^2)`, together with the uniform bound `(1-c)^2/(4Bc)`, at `c=0.1`. The zero curve `L_c=c` is intentionally omitted from the logarithmic axis. |
| `figures/annealed_noise.svg` | Product-estimator relative-variance upper bound `(1+v_B)^A-1` under the sufficient choice `B = ceil(2 A (1-c)^2/(4c))`, with the theorem ceiling `exp(1/kappa)-1`, `kappa=2`, `c=0.1`. |
| `figures/depth_components.svg` | Width-dependent terms in the MPBRI parallel-depth bound. `A`, `B`, and `Q` contribute logarithmic reduction/selection depth; local population width `N` contributes no per-particle depth term in the absence of a global reduction. |
| `figures/annealing_bound.svg` | Normalized optimizer-concentration term `r_delta^A` for `r_delta = 0.95, 0.85, 0.70`. |

The manuscript PDF versions are included as `figures/*.pdf`.

## Reproduce the figures

Install a stable Rust toolchain, then from the repository root run:

```text
cargo run --release
```

The command overwrites the four SVG files in `figures` deterministically. No external crates, Python packages, plotting libraries, datasets, network access, random seeds, or baseline implementations are required.

## Mathematical provenance

The figures visualize only identities or bounds proved in the manuscript:

1. Positive-floor likelihood noise: for `X in [c,1]`,
   `Var(X) <= (1-L_c)(L_c-c)`, which yields the displayed single-group relative-noise bounds.
2. Independent annealing groups: the exact product identity is
   `Var(Lhat_{c,A,B}) / L_c^(2A) = (1+v_B)^A - 1`.
3. Sufficient replica scaling: `B >= kappa A (1-c)^2/(4c)` gives a uniform upper bound `exp(1/kappa)-1`.
4. Parallel depth: the width-dependent part is `log B + log A + log Q`; the base term `D_phi + H(D_pi+D_P) + r d_theta` is held fixed in the figure.
5. Optimization concentration: the normalized term is `r_delta^A`, where the theorem defines `r_delta=(L_c^star-delta)/(L_c^star-delta/2) < 1`.

## Repository layout

```text
MPBRI_Rust_Figure_Reproducibility/
  Cargo.toml
  README.md
  src/
    main.rs
  figures/
    relative_noise.svg
    relative_noise.pdf
    annealed_noise.svg
    annealed_noise.pdf
    depth_components.svg
    depth_components.pdf
    annealing_bound.svg
    annealing_bound.pdf
```

There are intentionally **no hidden dot-files or dot-directories** in the repository.

## Scope

This repository is for **reproducibility of the theoretical MPBRI method figures**. It does not contain the baseline experiments, because those figures are not empirical baseline results and require no training data. The numerical values plotted here are generated directly from the manuscript's proved formulas.
