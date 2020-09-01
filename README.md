# disquiet

Ray tracer.

## TODO

Various things to try in no particular order:
- Camera
- NEE
- BVH (SBVH https://www.nvidia.ca/docs/IO/77714/sbvh.pdf)
- MIS
- Drop every shape but triangles
- Metropolis light transport
- Bidirectional path tracing
- Error-correcting float
- Textures
- Store texture pixels as a Hilbert curve
- SIMD
- Scene loader
- BRDFs:
  - Microfacet
    - Disney principled
    - NDFs: Trowbridge-Reitz, Beckmann
  - Cook-Torrance
  - Oren-Nayar
  - Anisotropic
  - Transmission
- Environment map importance sampling
- Instancing
- Animation
- Spectral rendering
- Node-based shaders
- Camera apertures
- Fractals
- DoF
- Stratified sampling
- Russian roulette
- Blue noise
- Normal maps
- Subsurface scattering
- Different samplers
- Denoising (OIDN?)
- Volumetric rendering
- Transparent objects
- Align vectors to cache lines 
- Two-level grids
- Various tone-mappers
- Light sampling
- GPU port, nvcc doesn't support rust though :(
- Interactive window