---
## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| urlId | string | query | Sí |  |

## Respuesta

Devuelve: [`GetPageByURLIdAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_page_by_urlid_api_response.go)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de GetPageByURLId'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	urlId := "urlId_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetPageByURLId(context.Background()).TenantId(tenantId).UrlId(urlId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetPageByURLId``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// respuesta de `GetPageByURLId`: GetPageByURLIdAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetPageByURLId`: %v\n", resp)
}
[inline-code-end]

---