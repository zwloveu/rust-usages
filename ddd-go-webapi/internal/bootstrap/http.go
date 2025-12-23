package bootstrap

import (
	"context"

	"github.com/zwloveu/rust-usages/ddd-go-webapi/internal/adapter/http"
	"github.com/zwloveu/rust-usages/ddd-go-webapi/internal/resource"
)

func RunAPI(ctx context.Context, appState *resource.AppState) error {
	repositories := NewRepositories(appState)

	applications := NewApplications(appState, repositories)

	handlers := NewHandlers(applications)

	router := http.NewRouter()

	http.RegisterUserRoutes(router, handlers.UserHandler)

	return http.Start(ctx, appState, router)
}
