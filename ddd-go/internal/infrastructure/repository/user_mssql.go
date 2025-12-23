package repository

import (
	"database/sql"

	domainUser "github.com/zwloveu/rust-usages/ddd-go-webapi/internal/domain/user"
	"github.com/zwloveu/rust-usages/ddd-go-webapi/internal/resource"
)

type UserMsSQLRepository struct {
	db *sql.DB
}

func NewUserMsSQLRepository(appState *resource.AppState) *UserMsSQLRepository {
	return &UserMsSQLRepository{
		db: appState.DB,
	}
}

func (repo *UserMsSQLRepository) FindAll() ([]*domainUser.User, error) {
	rows, err := repo.db.Query("SELECT id, name FROM users")
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var users []*domainUser.User
	for rows.Next() {
		var user domainUser.User
		if err := rows.Scan(&user.ID, &user.Name); err != nil {
			return nil, err
		}
		users = append(users, &user)
	}

	return users, nil
}
