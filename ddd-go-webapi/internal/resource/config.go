package resource

type Config struct {
	DB struct {
		Server   string
		User     string
		Password string
		Database string
	}
	HTTP struct {
		Addr string
	}
	Common struct {
		UseMemoryData bool
	}
}

func NewConfig() *Config {
	cfg := &Config{}

	cfg.DB.Server = "localhost"
	cfg.DB.User = "root"
	cfg.DB.Password = "password"
	cfg.DB.Database = "sample"

	cfg.HTTP.Addr = ":9527"

	cfg.Common.UseMemoryData = true

	return cfg
}
