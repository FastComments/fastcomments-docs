---
Past commenters on the page who are NOT currently online. Sorted by displayName.
Use this after exhausting /users/online to render a "Miembros" section.
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName}
index from afterName forward via $gt, no $skip cost.

## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identificador de la URL de la página (limpiado en el servidor). |
| afterName | string | query | No | Cursor: pase nextAfterName de la respuesta anterior. |
| afterUserId | string | query | No | Desempate de cursor: pase nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates por nombre no eliminen entradas. |

## Respuesta

Devuelve: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_offline_response.go)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de GetOfflineUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	urlId := "urlId_example" // string | Identificador de la URL de la página (limpiado en el servidor).
	afterName := "afterName_example" // string | Cursor: pase nextAfterName de la respuesta anterior. (opcional)
	afterUserId := "afterUserId_example" // string | Desempate de cursor: pase nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates por nombre no eliminen entradas. (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOfflineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOfflineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// respuesta de `GetOfflineUsers`: PageUsersOfflineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOfflineUsers`: %v\n", resp)
}
[inline-code-end]

---