package main

import (
	"log"
	"os"
	"os/signal"
	"syscall"

	"github.com/zwloveu/rust-usages/ddd-go-webapi/internal/bootstrap"
	"github.com/zwloveu/rust-usages/ddd-go-webapi/internal/resource"
)

func main() {
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

	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, syscall.SIGINT, syscall.SIGTERM)
	go func() {
		<-sigChan
		log.Println("received exit signal, shutting down server...")
		os.Exit(0) // trigger the exit of main then execute defer statements
	}()

	log.Println("starting API server...")
	if err := bootstrap.RunAPI(appState); err != nil {
		log.Fatalf("Failed to start API：%v", err)
	}
}
