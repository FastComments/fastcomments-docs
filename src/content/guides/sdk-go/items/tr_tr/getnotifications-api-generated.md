## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| userId | string | query | Hayır |  |
| urlId | string | query | Hayır |  |
| fromCommentId | string | query | Hayır |  |
| viewed | boolean | query | Hayır |  |
| type | string | query | Hayır |  |
| skip | number | query | Hayır |  |

## Yanıt

Döndürür: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_notifications_200_response.go)

## Örnek

[inline-code-attrs-start title = 'GetNotifications Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (isteğe bağlı)
	urlId := "urlId_example" // string |  (isteğe bağlı)
	fromCommentId := "fromCommentId_example" // string |  (isteğe bağlı)
	viewed := true // bool |  (isteğe bağlı)
	type_ := "type__example" // string |  (isteğe bağlı)
	skip := float64(1.2) // float64 |  (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetNotifications(context.Background()).TenantId(tenantId).UserId(userId).UrlId(urlId).FromCommentId(fromCommentId).Viewed(viewed).Type_(type_).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetNotifications`'ten dönen yanıt: GetNotifications200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetNotifications`: %v\n", resp)
}
[inline-code-end]

---