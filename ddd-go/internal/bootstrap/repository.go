package bootstrap

import (
	domainUser "github.com/zwloveu/rust-usages/ddd-go-webapi/internal/domain/user"
	infraRepo "github.com/zwloveu/rust-usages/ddd-go-webapi/internal/infrastructure/repository"
	"github.com/zwloveu/rust-usages/ddd-go-webapi/internal/resource"
)

type Repositories struct {
	UserRepoMemory domainUser.Repository
	UserRepoMsSQL  domainUser.Repository
}

func NewRepositories(appState *resource.AppState) *Repositories {
	return &Repositories{
		UserRepoMemory: infraRepo.NewUserMemoryRepository(),
		UserRepoMsSQL:  infraRepo.NewUserMsSQLRepository(appState),
	}
}
