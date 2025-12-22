package http

import (
	"github.com/gin-gonic/gin"
	"github.com/zwloveu/rust-usages/ddd-go-webapi/internal/resource"
)

func Start(appState *resource.AppState, router *gin.Engine) error {
	return router.Run(appState.Config.HTTP.Addr)
}
