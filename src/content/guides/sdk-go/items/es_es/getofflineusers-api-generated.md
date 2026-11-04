Comentadores anteriores en la página que NO están actualmente en línea. Ordenados por displayName.
Úselo después de agotar /users/online para renderizar una sección "Members".
Paginación por cursor sobre commenterName: el servidor recorre el índice parcial {tenantId, urlId, commenterName}
desde afterName hacia adelante mediante $gt, sin coste de $skip.

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|--------|------|-----------|-----------|-------------|
| tenantId | string | path | Sí |  |
| urlId | string | query | Sí | Identificador de la URL de la página (limpiado en el servidor). |
| afterName | string | query | No | Cursor: pase nextAfterName de la respuesta anterior. |
| afterUserId | string | query | No | Desempate del cursor: pase nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates de nombre no descarten entradas. |

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
	afterName := "afterName_example" // string | Cursor: pase nextAfterName de la respuesta anterior. (optional)
	afterUserId := "afterUserId_example" // string | Desempate del cursor: pase nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates de nombre no descarten entradas. (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOfflineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOfflineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetOfflineUsers`: PageUsersOfflineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOfflineUsers`: %v\n", resp)
}
[inline-code-end]