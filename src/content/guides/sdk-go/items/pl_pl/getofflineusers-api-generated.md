---
Poprzedni komentujący na stronie, którzy NIE są obecnie online. Posortowane według displayName.
Użyj tego po wyczerpaniu /users/online, aby wyświetlić sekcję "Członkowie".
Paginacja kursorowa po commenterName: serwer przegląda częściowy indeks {tenantId, urlId, commenterName} zaczynając od afterName i idąc w przód za pomocą $gt, bez kosztu $skip.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identyfikator URL strony (oczyszczony po stronie serwera). |
| afterName | string | query | No | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi. |
| afterUserId | string | query | No | Rozstrzygacz kursora: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy afterName jest ustawione, aby przy remisach nazw wpisy nie zostały pominięte. |

## Response

Zwraca: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_offline_response.go)

## Example

[inline-code-attrs-start title = 'Przykład GetOfflineUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Identyfikator URL strony (oczyszczony po stronie serwera).
	afterName := "afterName_example" // string | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi. (opcjonalne)
	afterUserId := "afterUserId_example" // string | Rozstrzygacz kursora: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy afterName jest ustawione, aby przy remisach nazw wpisy nie były pomijane. (opcjonalne)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOfflineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOfflineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `GetOfflineUsers`: PageUsersOfflineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOfflineUsers`: %v\n", resp)
}
[inline-code-end]

---