#include "pxr/base/tf/token.h"

#define CPPMM_IGNORE __attribute__((annotate("cppmm:ignore")))
#define CPPMM_RENAME(x) __attribute__((annotate("cppmm:rename:" #x)))
#define CPPMM_OPAQUEPTR __attribute__((annotate("cppmm:opaqueptr")))

namespace cppmm_bind {
namespace PXR_INTERNAL_NS {

namespace pxr = PXR_INTERNAL_NS;

class TfToken {
public:
    //TfToken() CPPMM_RENAME(ctor);
    size_t Hash() const;
} CPPMM_OPAQUEBYTES;

}
} // namespace cppmm_bind
