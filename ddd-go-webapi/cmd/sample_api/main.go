package main

import (
	"log"

	"github.com/zwloveu/rust-usages/ddd-go-webapi/internal/bootstrap"
	"github.com/zwloveu/rust-usages/ddd-go-webapi/internal/resource"
)

func main() {
	cfg := resource.NewConfig()
	appState, err := resource.NewAppState(cfg)
	if err != nil {
		log.Fatalf("failed to init config：%v", err)
	}

	if err := bootstrap.RunAPI(appState); err != nil {
		log.Fatalf("Failed to start API：%v", err)
	}
}
