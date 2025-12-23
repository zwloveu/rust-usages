package main

import (
	"context"
	"log"
	"os/signal"
	"sync"
	"syscall"

	"github.com/zwloveu/rust-usages/ddd-go-webapi/internal/bootstrap"
	"github.com/zwloveu/rust-usages/ddd-go-webapi/internal/resource"
)

func main() {
	ctx, stop := signal.NotifyContext(context.Background(), syscall.SIGINT, syscall.SIGTERM)
	defer stop()

	cfg := resource.NewConfig()
	appState, err := resource.NewAppState(cfg)
	if err != nil {
		log.Fatalf("failed to init config：%v", err)
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
		bootstrap.RunAPI(ctx, appState)
	})

	// Wait for CTRL+C
	<-ctx.Done()
	log.Println("received exit signal, shutting down...")

	wg.Wait()
}
