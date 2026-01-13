## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | zapytanie | Tak |  |
| userId | string | zapytanie | Nie |  |
| urlId | string | zapytanie | Nie |  |
| fromCommentId | string | zapytanie | Nie |  |
| viewed | boolean | zapytanie | Nie |  |
| type | string | zapytanie | Nie |  |
| skip | number | zapytanie | Nie |  |

## Odpowiedź

Zwraca: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_notifications_200_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład GetNotifications'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (opcjonalne)
	urlId := "urlId_example" // string |  (opcjonalne)
	fromCommentId := "fromCommentId_example" // string |  (opcjonalne)
	viewed := true // bool |  (opcjonalne)
	type_ := "type__example" // string |  (opcjonalne)
	skip := float64(1.2) // float64 |  (opcjonalne)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetNotifications(context.Background()).TenantId(tenantId).UserId(userId).UrlId(urlId).FromCommentId(fromCommentId).Viewed(viewed).Type_(type_).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `GetNotifications`: GetNotifications200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetNotifications`: %v\n", resp)
}
[inline-code-end]

---