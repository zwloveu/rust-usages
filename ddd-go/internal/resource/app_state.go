package resource

import "database/sql"

type AppState struct {
	DB     *sql.DB
	Config *Config
}

func NewAppState(cfg *Config) (*AppState, error) {
	db, err := NewDB(cfg)
	if err != nil {
		return nil, err
	}

	return &AppState{
		DB:     db,
		Config: cfg,
	}, nil
}
