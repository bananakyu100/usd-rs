#include "pxr/base/tf/token.h"

#define CPPMM_IGNORE __attribute__((annotate("cppmm:ignore")))
#define CPPMM_RENAME(x) __attribute__((annotate("cppmm:rename:" #x)))
#define CPPMM_OPAQUEPTR __attribute__((annotate("cppmm:opaqueptr")))

namespace cppmm_bind {
PXR_NAMESPACE_OPEN_SCOPE

namespace pxr = PXR_INTERNAL_NS;

class TfToken {
public:
    TfToken() CPPMM_RENAME(ctor);
} CPPMM_OPAQUEBYTES;

PXR_NAMESPACE_CLOSE_SCOPE
} // namespace cppmm_bind
