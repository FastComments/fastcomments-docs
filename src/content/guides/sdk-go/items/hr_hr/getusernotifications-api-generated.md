## Parametri

| Name | Type | Location | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| urlId | string | query | Ne | Koristi se za određivanje je li trenutna stranica pretplaćena. |
| pageSize | integer | query | Ne |  |
| afterId | string | query | Ne |  |
| includeContext | boolean | query | Ne |  |
| afterCreatedAt | integer | query | Ne |  |
| unreadOnly | boolean | query | Ne |  |
| dmOnly | boolean | query | Ne |  |
| noDm | boolean | query | Ne |  |
| includeTranslations | boolean | query | Ne |  |
| includeTenantNotifications | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_my_notifications_response.go)

## Primjer

[inline-code-attrs-start title = 'Primjer GetUserNotifications'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Koristi se za određivanje je li trenutna stranica pretplaćena. (neobavezno)
	pageSize := int32(56) // int32 |  (neobavezno)
	afterId := "afterId_example" // string |  (neobavezno)
	includeContext := true // bool |  (neobavezno)
	afterCreatedAt := int64(789) // int64 |  (neobavezno)
	unreadOnly := true // bool |  (neobavezno)
	dmOnly := true // bool |  (neobavezno)
	noDm := true // bool |  (neobavezno)
	includeTranslations := true // bool |  (neobavezno)
	includeTenantNotifications := true // bool |  (neobavezno)
	sso := "sso_example" // string |  (neobavezno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserNotifications(context.Background()).TenantId(tenantId).UrlId(urlId).PageSize(pageSize).AfterId(afterId).IncludeContext(includeContext).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).IncludeTranslations(includeTranslations).IncludeTenantNotifications(includeTenantNotifications).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor iz `GetUserNotifications`: GetMyNotificationsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserNotifications`: %v\n", resp)
}
[inline-code-end]