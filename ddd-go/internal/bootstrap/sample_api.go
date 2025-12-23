package bootstrap

import (
	"context"
	"fmt"
	"log"
	"os/signal"
	"sync"
	"syscall"

	"github.com/zwloveu/rust-usages/ddd-go-webapi/internal/adapter/http"
	"github.com/zwloveu/rust-usages/ddd-go-webapi/internal/resource"
)

func RunSampleAPI() error {
	ctx, stop := signal.NotifyContext(context.Background(), syscall.SIGINT, syscall.SIGTERM)
	defer stop()

	cfg := resource.NewConfig()
	appState, err := resource.NewAppState(cfg)
	if err != nil {
		return fmt.Errorf("failed to init config：%w", err)
	}

	defer func() {
		if err := appState.DB.Close(); err != nil {
			log.Printf("warning: failed to close db connection pool: %v", err)
		} else {
			log.Println("db connection pool closed gracefully")
		}
	}()

	var wg sync.WaitGroup

	wg.Go(func() {
		if err := startAPIServer(ctx, appState); err != nil {
			log.Printf("API failed to start: %v", err)
		}
	})

	// Wait for CTRL+C
	<-ctx.Done()
	log.Println("received exit signal, shutting down...")

	wg.Wait()

	return nil
}

func startAPIServer(ctx context.Context, appState *resource.AppState) error {
	repositories := NewRepositories(appState)

	applications := NewApplications(appState, repositories)

	handlers := NewHandlers(applications)

	router := http.NewRouter()

	http.RegisterUserRoutes(router, handlers.UserHandler)

	return http.Start(ctx, appState, router)
}
