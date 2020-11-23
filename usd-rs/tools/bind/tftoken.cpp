#include "pxr/base/tf/token.h"

#define CPPMM_IGNORE __attribute__((annotate("cppmm:ignore")))
#define CPPMM_RENAME(x) __attribute__((annotate("cppmm:rename:" #x)))
#define CPPMM_OPAQUEBYTES __attribute__((annotate("cppmm:opaquebytes")))

namespace cppmm_bind {
PXR_NAMESPACE_OPEN_SCOPE

namespace pxr = PXR_INTERNAL_NS;

class TfToken {
    //constexpr TfToken() noexcept {};
} CPPMM_OPAQUEBYTES;

PXR_NAMESPACE_CLOSE_SCOPE
} // namespace cppmm_bind
