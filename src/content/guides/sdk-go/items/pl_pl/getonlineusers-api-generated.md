Obecnie online widzowie strony: osoby, których sesja websocket jest aktualnie subskrybująca stronę.
Zwraca anonCount + totalCount (subskrybenci w całym pokoju, wliczając anonimowych widzów, których nie wyliczamy).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identyfikator URL strony (oczyszczany po stronie serwera). |
| afterName | string | query | No | Cursor: przekaż nextAfterName z poprzedniej odpowiedzi. |
| afterUserId | string | query | No | Rozstrzygacz kursora: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy afterName jest ustawione, aby remisy nazw nie powodowały pominięcia wpisów. |

## Response

Zwraca: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_online_response.go)

## Example

[inline-code-attrs-start title = 'Przykład GetOnlineUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Identyfikator URL strony (oczyszczany po stronie serwera).
	afterName := "afterName_example" // string | Cursor: przekaż nextAfterName z poprzedniej odpowiedzi. (opcjonalne)
	afterUserId := "afterUserId_example" // string | Rozstrzygacz kursora: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy afterName jest ustawione, aby remisy nazw nie powodowały pominięcia wpisów. (opcjonalne)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOnlineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOnlineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetOnlineUsers`: PageUsersOnlineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOnlineUsers`: %v\n", resp)
}
[inline-code-end]