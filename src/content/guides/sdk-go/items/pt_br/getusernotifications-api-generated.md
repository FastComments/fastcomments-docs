## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| urlId | string | query | Não | Usado para determinar se a página atual está inscrita. |
| pageSize | integer | query | Não |  |
| afterId | string | query | Não |  |
| includeContext | boolean | query | Não |  |
| afterCreatedAt | integer | query | Não |  |
| unreadOnly | boolean | query | Não |  |
| dmOnly | boolean | query | Não |  |
| noDm | boolean | query | Não |  |
| includeTranslations | boolean | query | Não |  |
| includeTenantNotifications | boolean | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_my_notifications_response.go)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de GetUserNotifications'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Usado para determinar se a página atual está inscrita. (opcional)
	pageSize := int32(56) // int32 |  (opcional)
	afterId := "afterId_example" // string |  (opcional)
	includeContext := true // bool |  (opcional)
	afterCreatedAt := int64(789) // int64 |  (opcional)
	unreadOnly := true // bool |  (opcional)
	dmOnly := true // bool |  (opcional)
	noDm := true // bool |  (opcional)
	includeTranslations := true // bool |  (opcional)
	includeTenantNotifications := true // bool |  (opcional)
	sso := "sso_example" // string |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserNotifications(context.Background()).TenantId(tenantId).UrlId(urlId).PageSize(pageSize).AfterId(afterId).IncludeContext(includeContext).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).IncludeTranslations(includeTranslations).IncludeTenantNotifications(includeTenantNotifications).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetUserNotifications`: GetMyNotificationsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserNotifications`: %v\n", resp)
}
[inline-code-end]

---