package resource

import (
	"database/sql"
	"fmt"

	_ "github.com/denisenkom/go-mssqldb"
)

func NewDB(cfg *Config) (*sql.DB, error) {
	connString := fmt.Sprintf("server=%s;user id=%s;password=%s;database=%s;",
		cfg.DB.Server, cfg.DB.User, cfg.DB.Password, cfg.DB.Database)

	db, err := sql.Open("sqlserver", connString)
	if err != nil {
		return nil, err
	}

	if !cfg.Common.UseMemoryData {
		if err := db.Ping(); err != nil {
			closeErr := db.Close()
			if closeErr != nil {
				fmt.Printf("warning: failed to close invalid db pool: %v\n", closeErr)
			}
			return nil, fmt.Errorf("failed to ping database: %w", err)
		}
	}

	return db, nil
}
