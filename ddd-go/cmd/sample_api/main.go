package main

import (
	"fmt"
	"os"

	"github.com/zwloveu/rust-usages/ddd-go-webapi/internal/bootstrap"
)

func main() {
	if err := bootstrap.RunSampleAPI(); err != nil {
		fmt.Printf("failed to run application: %v\n", err)
		os.Exit(1)
	}
}
