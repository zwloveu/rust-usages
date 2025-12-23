package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"os/signal"
	"sync"
	"syscall"

	"github.com/zwloveu/rust-usages/ddd-go-webapi/internal/bootstrap"
	"github.com/zwloveu/rust-usages/ddd-go-webapi/internal/resource"
)

func main() {
	if err := run(); err != nil {
		fmt.Printf("failed to run application: %v\n", err)
		os.Exit(1)
	}
}

func run() error {
	ctx, stop := signal.NotifyContext(context.Background(), syscall.SIGINT, syscall.SIGTERM)
	defer stop()

	cfg := resource.NewConfig()
	appState, err := resource.NewAppState(cfg)
	if err != nil {
		return fmt.Errorf("failed to init config：%v", err)
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
		if err := bootstrap.RunAPI(ctx, appState); err != nil {
			log.Printf("API failed to start: %v", err)
		}
	})

	// Wait for CTRL+C
	<-ctx.Done()
	log.Println("received exit signal, shutting down...")

	wg.Wait()

	return nil
}
