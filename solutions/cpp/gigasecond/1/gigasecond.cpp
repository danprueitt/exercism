#include "gigasecond.h"

namespace gigasecond {
    boost::posix_time::ptime advance(boost::posix_time::ptime time) {
        return time.operator+(boost::posix_time::time_duration(0, 0, 1000000000));
    }
}  // namespace gigasecond
