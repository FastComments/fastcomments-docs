## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Nee | Wordt gebruikt om te bepalen of de huidige pagina is geabonneerd. |
| pageSize | integer | query | Nee |  |
| afterId | string | query | Nee |  |
| includeContext | boolean | query | Nee |  |
| afterCreatedAt | integer | query | Nee |  |
| unreadOnly | boolean | query | Nee |  |
| dmOnly | boolean | query | Nee |  |
| noDm | boolean | query | Nee |  |
| includeTranslations | boolean | query | Nee |  |
| includeTenantNotifications | boolean | query | Nee |  |
| sso | string | query | Nee |  |

## Antwoord

Retourneert: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_my_notifications_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'GetUserNotifications Voorbeeld'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Wordt gebruikt om te bepalen of de huidige pagina is geabonneerd. (optioneel)
	pageSize := int32(56) // int32 |  (optioneel)
	afterId := "afterId_example" // string |  (optioneel)
	includeContext := true // bool |  (optioneel)
	afterCreatedAt := int64(789) // int64 |  (optioneel)
	unreadOnly := true // bool |  (optioneel)
	dmOnly := true // bool |  (optioneel)
	noDm := true // bool |  (optioneel)
	includeTranslations := true // bool |  (optioneel)
	includeTenantNotifications := true // bool |  (optioneel)
	sso := "sso_example" // string |  (optioneel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserNotifications(context.Background()).TenantId(tenantId).UrlId(urlId).PageSize(pageSize).AfterId(afterId).IncludeContext(includeContext).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).IncludeTranslations(includeTranslations).IncludeTenantNotifications(includeTenantNotifications).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// antwoord van `GetUserNotifications`: GetMyNotificationsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserNotifications`: %v\n", resp)
}
[inline-code-end]