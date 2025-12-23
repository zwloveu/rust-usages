package http

import (
	"github.com/gin-gonic/gin"
)

func RegisterUserRoutes(router *gin.Engine, handler *UserHandler) {
	g := router.Group("/users")
	g.GET("/", handler.GetUsers)
}
