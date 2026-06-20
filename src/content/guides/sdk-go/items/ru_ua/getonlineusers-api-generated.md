Сейчас онлайн просматривающие страницу: люди, чья websocket-сессия в данный момент подписана на эту страницу.
Возвращает anonCount + totalCount (все подписчики комнаты, включая анонимных зрителей, которых мы не перечисляем).

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL страницы (очищается на стороне сервера). |
| afterName | string | query | No | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | No | Дополнительный курсор для разрешения равенства: передайте nextAfterUserId из предыдущего ответа. Обязательно, когда задан afterName, чтобы при совпадении имён записи не терялись. |

## Ответ

Возвращает: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_online_response.go)

## Пример

[inline-code-attrs-start title = 'Пример GetOnlineUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Идентификатор URL страницы (очищается на стороне сервера).
	afterName := "afterName_example" // string | Курсор: передайте nextAfterName из предыдущего ответа. (необязательно)
	afterUserId := "afterUserId_example" // string | Дополнительный курсор для разрешения равенства: передайте nextAfterUserId из предыдущего ответа. Обязательно, когда задан afterName, чтобы при совпадении имён записи не терялись. (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOnlineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOnlineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// ответ от `GetOnlineUsers`: PageUsersOnlineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOnlineUsers`: %v\n", resp)
}
[inline-code-end]