---
В данный момент онлайн-зрители страницы: люди, чьи websocket-сессии в настоящее время подписаны на страницу.
Возвращает anonCount + totalCount (подписчики всей комнаты, включая анонимных зрителей, которых мы не перечисляем).

## Параметры

| Имя | Тип | Location | Обязательный | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор URL страницы (очищается на стороне сервера). |
| afterName | string | query | Нет | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | Нет | Курсор-тайбрейкер: передайте nextAfterUserId из предыдущего ответа. Обязательно, когда afterName установлен, чтобы записи с одинаковыми именами не пропадали. |

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
	afterUserId := "afterUserId_example" // string | Курсор-тайбрейкер: передайте nextAfterUserId из предыдущего ответа. Обязательно, когда afterName установлен, чтобы записи с одинаковыми именами не пропадали. (необязательно)

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

---