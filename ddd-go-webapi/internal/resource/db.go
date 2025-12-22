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

	return db, nil
}
