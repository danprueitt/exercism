#include "space_age.h"

namespace space_age {

    space_age::space_age(const double seconds) : AGE_SECONDS(seconds) {}

    double space_age::seconds() const {
        return AGE_SECONDS;
    }

    double space_age::on_earth() const {
        return AGE_SECONDS / SECONDS_IN_EARTH_YEAR;
    }

    double space_age::on_mercury() const {
        return get_earth_year_orbit("mercury");
    }

    double space_age::on_venus() const {
        return get_earth_year_orbit("venus");
    }

    double space_age::on_mars() const {
        return get_earth_year_orbit("mars");
    }

    double space_age::on_jupiter() const {
        return get_earth_year_orbit("jupiter");
    }

    double space_age::on_saturn() const {
        return get_earth_year_orbit("saturn");
    }

    double space_age::on_uranus() const {
        return get_earth_year_orbit("uranus");
    }

    double space_age::on_neptune() const {
        return get_earth_year_orbit("neptune");
    }

    double space_age::get_earth_year_orbit(const std::string &planet_name) const {
        return AGE_SECONDS / SECONDS_IN_EARTH_YEAR / YEARS_TO_ORBIT_EARTH.at(planet_name);
    }

}  // namespace space_age
