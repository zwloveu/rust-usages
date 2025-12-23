package repository

import (
	domainUser "github.com/zwloveu/rust-usages/ddd-go-webapi/internal/domain/user"
)

type UserMemoryRepository struct {
	users []*domainUser.User
}

func NewUserMemoryRepository() *UserMemoryRepository {
	return &UserMemoryRepository{
		users: []*domainUser.User{
			{
				ID:   1,
				Name: "John Doe",
			},
			{
				ID:   2,
				Name: "Jane Doe",
			},
		},
	}
}

func (repo *UserMemoryRepository) FindAll() ([]*domainUser.User, error) {
	return repo.users, nil
}
