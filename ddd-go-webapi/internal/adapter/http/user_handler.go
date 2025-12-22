package http

import (
	netHttp "net/http"

	"github.com/gin-gonic/gin"
	appUser "github.com/zwloveu/rust-usages/ddd-go-webapi/internal/application/user"
)

type UserHandler struct {
	QueryService *appUser.QueryService
}

func NewUserHandler(queryService *appUser.QueryService) *UserHandler {
	return &UserHandler{
		QueryService: queryService,
	}
}

func (h *UserHandler) GetUsers(c *gin.Context) {
	users, err := h.QueryService.GetAllUsers()

	if err != nil {
		c.JSON(netHttp.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	c.JSON(netHttp.StatusOK, users)
}
