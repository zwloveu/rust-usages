package bootstrap

import "github.com/zwloveu/rust-usages/ddd-go-webapi/internal/adapter/http"

type Handlers struct {
	UserHandler *http.UserHandler
}

func NewHandlers(applications *Applications) *Handlers {
	return &Handlers{
		UserHandler: http.NewUserHandler(applications.UserQuery),
	}
}
