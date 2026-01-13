## Parâmetros

| Nome | Tipo | Location | Obrigatório | Descrição |
|------|------|----------|------------|-----------|
| tenantId | string | query | Sim |  |
| userId | string | query | Não |  |
| urlId | string | query | Não |  |
| fromCommentId | string | query | Não |  |
| viewed | boolean | query | Não |  |
| type | string | query | Não |  |
| skip | number | query | Não |  |

## Resposta

Retorna: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_notifications_200_response.go)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de GetNotifications'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (opcional)
	urlId := "urlId_example" // string |  (opcional)
	fromCommentId := "fromCommentId_example" // string |  (opcional)
	viewed := true // bool |  (opcional)
	type_ := "type__example" // string |  (opcional)
	skip := float64(1.2) // float64 |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetNotifications(context.Background()).TenantId(tenantId).UserId(userId).UrlId(urlId).FromCommentId(fromCommentId).Viewed(viewed).Type_(type_).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// resposta de `GetNotifications`: GetNotifications200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetNotifications`: %v\n", resp)
}
[inline-code-end]