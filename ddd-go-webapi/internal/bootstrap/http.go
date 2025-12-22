package bootstrap

import (
	"github.com/zwloveu/rust-usages/ddd-go-webapi/internal/adapter/http"
	"github.com/zwloveu/rust-usages/ddd-go-webapi/internal/resource"
)

func RunAPI(appState *resource.AppState) error {
	repositories := NewRepositories(appState)

	applications := NewApplications(appState, repositories)

	handlers := NewHandlers(applications)

	router := http.NewRouter()

	http.RegisterUserRoutes(router, handlers.UserHandler)

	return http.Start(appState, router)
}
