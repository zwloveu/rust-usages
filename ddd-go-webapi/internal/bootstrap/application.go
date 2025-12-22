package bootstrap

import (
	appUser "github.com/zwloveu/rust-usages/ddd-go-webapi/internal/application/user"
	domainUser "github.com/zwloveu/rust-usages/ddd-go-webapi/internal/domain/user"
	"github.com/zwloveu/rust-usages/ddd-go-webapi/internal/resource"
)

type Applications struct {
	UserQuery *appUser.QueryService
}

func NewApplications(appState *resource.AppState, repositories *Repositories) *Applications {
	var user_repo domainUser.Repository
	if appState.Config.Common.UseMemoryData {
		user_repo = repositories.UserRepoMemory
	} else {
		user_repo = repositories.UserRepoMsSQL
	}

	return &Applications{
		UserQuery: appUser.NewQueryService(user_repo),
	}
}
