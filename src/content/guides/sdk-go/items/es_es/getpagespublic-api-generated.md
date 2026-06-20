Lista páginas para un tenant. Usado por el cliente de escritorio FChat para rellenar su lista de salas.
Requiere que `enableFChat` sea true en la configuración personalizada resuelta para cada página.
Las páginas que requieren SSO se filtran según el acceso de grupo del usuario que realiza la solicitud.

## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| cursor | string | query | No | Cursor opaco de paginación devuelto como `nextCursor` en una solicitud anterior. Vinculado al mismo `sortBy`. |
| limit | integer | query | No | 1..200, predeterminado 50 |
| q | string | query | No | Filtro opcional por prefijo de título sin distinguir mayúsculas. |
| sortBy | string | query | No | Orden. `updatedAt` (predeterminado, más recientes primero), `commentCount` (más comentarios primero), o `title` (alfabético). |
| hasComments | boolean | query | No | Si es true, solo devuelve páginas con al menos un comentario. |

## Respuesta

Devuelve: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_public_pages_response.go)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de GetPagesPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	cursor := "cursor_example" // string | Cursor opaco de paginación devuelto como `nextCursor` en una solicitud anterior. Vinculado al mismo `sortBy`. (opcional)
	limit := int32(56) // int32 | 1..200, predeterminado 50 (opcional)
	q := "q_example" // string | Filtro opcional por prefijo de título sin distinguir mayúsculas. (opcional)
	sortBy := openapiclient.PagesSortBy("updatedAt") // PagesSortBy | Orden. `updatedAt` (predeterminado, más recientes primero), `commentCount` (más comentarios primero), o `title` (alfabético). (opcional)
	hasComments := true // bool | Si es true, solo devuelve páginas con al menos un comentario. (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetPagesPublic(context.Background(), tenantId).Cursor(cursor).Limit(limit).Q(q).SortBy(sortBy).HasComments(hasComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetPagesPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// respuesta de `GetPagesPublic`: GetPublicPagesResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetPagesPublic`: %v\n", resp)
}
[inline-code-end]