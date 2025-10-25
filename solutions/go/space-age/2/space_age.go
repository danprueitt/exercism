package space

type Planet string

const (
	// The orbital period of the Earth in seconds
	EarthYearsInSeconds = 31557600
	// Mercury orbital period in Years
	MercuryOrbitalYears = 0.2408467
	// Venus orbital period in Years
	VenusOrbitalYears = 0.61519726
	// Mars orbital period in Years
	MarsOrbitalYears = 1.8808158
	// Jupiter orbital period in Years
	JupiterOrbitalYears = 11.862615
	// Saturn orbital period in Years
	SaturnOrbitalYears = 29.447498
	// Uranus orbital period in Years
	UranusOrbitalYears = 84.016846
	// Neptune orbital period in Years
	NeptuneOrbitalYears = 164.79132
)

// Age calculates the number of earth years on a planet
// for a given number of seconds
func Age(seconds float64, planet Planet) (period float64) {
	switch planet {
	case "Earth":
		period = secondsToEarthYears(seconds, 1)
	case "Mercury":
		period = secondsToEarthYears(seconds, MercuryOrbitalYears)
	case "Venus":
		period = secondsToEarthYears(seconds, VenusOrbitalYears)
	case "Mars":
		period = secondsToEarthYears(seconds, MarsOrbitalYears)
	case "Jupiter":
		period = secondsToEarthYears(seconds, JupiterOrbitalYears)
	case "Saturn":
		period = secondsToEarthYears(seconds, SaturnOrbitalYears)
	case "Uranus":
		period = secondsToEarthYears(seconds, UranusOrbitalYears)
	case "Neptune":
		period = secondsToEarthYears(seconds, NeptuneOrbitalYears)
	default:
		panic("unknown planet")
	}
	return
}

// secondsToEarthYears converts seconds to earth years
func secondsToEarthYears(seconds float64, orbitalPeriod float64) float64 {
	return seconds / (orbitalPeriod * EarthYearsInSeconds)
}
