Користувачі, які зараз онлайн на сторінці: люди, чиї websocket-сесії зараз підписані на цю сторінку.
Повертає anonCount + totalCount (підписники на кімнату, включаючи анонімних глядачів, яких ми не перераховуємо).

## Parameters

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Ідентифікатор URL сторінки (очищено на сервері). |
| afterName | string | query | No | Курсор: передайте nextAfterName з попередньої відповіді. |
| afterUserId | string | query | No | Додатковий курсор для розв'язання однакових імен: передайте nextAfterUserId з попередньої відповіді. Необхідно, коли встановлено afterName, щоб записи з однаковими іменами не втрачалися. |

## Response

Повертає: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_online_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад GetOnlineUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Ідентифікатор URL сторінки (очищено на сервері).
	afterName := "afterName_example" // string | Курсор: передайте nextAfterName з попередньої відповіді. (необов'язково)
	afterUserId := "afterUserId_example" // string | Додатковий курсор для розв'язання однакових імен: передайте nextAfterUserId з попередньої відповіді. Необхідно, коли встановлено afterName, щоб записи з однаковими іменами не втрачалися. (необов'язково)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOnlineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOnlineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь від `GetOnlineUsers`: PageUsersOnlineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOnlineUsers`: %v\n", resp)
}
[inline-code-end]