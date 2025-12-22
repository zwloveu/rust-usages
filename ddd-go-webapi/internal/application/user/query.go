package user

import "github.com/zwloveu/rust-usages/ddd-go-webapi/internal/domain/user"

type QueryService struct {
	Repo user.Repository
}

func NewQueryService(repo user.Repository) *QueryService {
	return &QueryService{
		Repo: repo,
	}
}

func (qs *QueryService) GetAllUsers() ([]*user.User, error) {
	return qs.Repo.FindAll()
}
