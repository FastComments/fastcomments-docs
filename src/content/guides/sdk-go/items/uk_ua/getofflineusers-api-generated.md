Попередні коментатори на сторінці, які НЕ зараз в мережі. Відсортовано за displayName.
Використовуйте це після вичерпання /users/online, щоб відобразити розділ "Учасники".
Пагінація курсором за commenterName: сервер проходить частковий індекс {tenantId, urlId, commenterName} від afterName уперед за допомогою $gt, без витрат $skip.

## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Page URL identifier (cleaned server-side). |
| afterName | string | query | No | Курсор: передайте nextAfterName з попередньої відповіді. |
| afterUserId | string | query | No | Курсор-тайбрекер: передайте nextAfterUserId з попередньої відповіді. Обов'язково, коли afterName встановлено, щоб зв'язки за іменем не призводили до втрати записів. |

## Відповідь

Повертає: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_offline_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад GetOfflineUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Ідентифікатор URL сторінки (очищається на сервері).
	afterName := "afterName_example" // string | Курсор: передайте nextAfterName з попередньої відповіді. (необов'язково)
	afterUserId := "afterUserId_example" // string | Курсор-тайбрекер: передайте nextAfterUserId з попередньої відповіді. Обов'язково, коли afterName встановлено, щоб зв'язки за іменем не призводили до втрати записів. (необов'язково)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOfflineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOfflineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь від `GetOfflineUsers`: PageUsersOfflineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOfflineUsers`: %v\n", resp)
}
[inline-code-end]