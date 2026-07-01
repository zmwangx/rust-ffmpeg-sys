// `hwcontext_cuda.h` #includes the CUDA SDK's <cuda.h> (unless CUDA_VERSION is
// already defined) purely to obtain the opaque `CUcontext` / `CUstream` handle
// types that `AVCUDADeviceContext` holds. We don't need to drag the entire CUDA-toolkit
// for 2 pointers so we predefine CUDA_VERSION and the two handle typedefs
#define CUDA_VERSION 12000
typedef struct CUctx_st *CUcontext;
typedef struct CUstream_st *CUstream;
#include <libavutil/hwcontext_cuda.h>
