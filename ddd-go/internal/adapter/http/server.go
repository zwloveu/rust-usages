package http

import (
	"context"
	"log"
	"time"

	netHttp "net/http"

	"github.com/gin-gonic/gin"
	"github.com/zwloveu/rust-usages/ddd-go-webapi/internal/resource"
)

func Start(ctx context.Context, appState *resource.AppState, router *gin.Engine) error {
	srv := &netHttp.Server{
		Addr:    appState.Config.HTTP.Addr,
		Handler: router,
	}

	go func() {
		if err := srv.ListenAndServe(); err != nil && err != netHttp.ErrServerClosed {
			log.Fatalf("listern: %s", err)
		}
	}()

	<-ctx.Done()
	log.Println("shutting down API server...")

	shutdownCtx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	return srv.Shutdown(shutdownCtx)
}
