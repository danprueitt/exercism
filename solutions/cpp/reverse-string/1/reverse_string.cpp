#include "reverse_string.h"

namespace reverse_string
{

    std::string reverse_string(const std::string s)
    {
        std::string rev = "";
        for (int i=s.length()-1; i >= 0; i--)
        {
            rev += s[i];
        }

        return rev;
    }

} // namespace reverse_string
