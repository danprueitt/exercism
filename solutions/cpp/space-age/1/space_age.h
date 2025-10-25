#if !defined(SPACE_AGE_H)
#define SPACE_AGE_H

#include <map>
#include <string>
#include "test/catch.hpp"

namespace space_age {
    class space_age {
    private:
        const double AGE_SECONDS;
    public:
        const double SECONDS_IN_EARTH_YEAR = 31557600;
        const std::map<std::string, double> YEARS_TO_ORBIT_EARTH = {
                std::pair<std::string, double>("mercury", 0.2408467),
                std::pair<std::string, double>("venus", 0.61519726),
                std::pair<std::string, double>("earth", 1.0),
                std::pair<std::string, double>("mars", 1.8808158),
                std::pair<std::string, double>("jupiter", 11.862615),
                std::pair<std::string, double>("saturn", 29.447498),
                std::pair<std::string, double>("uranus", 84.016846),
                std::pair<std::string, double>("neptune", 164.79132),
        };

        explicit space_age(double age_seconds);

        double get_earth_year_orbit(const std::string &planet_name) const;

        double seconds() const;

        double on_earth() const;

        double on_mercury() const;

        double on_venus() const;

        double on_mars() const;

        double on_jupiter() const;

        double on_saturn() const;

        double on_uranus() const;

        double on_neptune() const;
    };

}  // namespace space_age

#endif // SPACE_AGE_H