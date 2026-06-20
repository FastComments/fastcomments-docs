Прошлые комментаторы на странице, которые в настоящее время НЕ в сети. Отсортировано по displayName.
Используйте это после исчерпания /users/online, чтобы отобразить раздел «Участники».
Постраничная пагинация курсором по commenterName: сервер просматривает частичный {tenantId, urlId, commenterName}
индекс от afterName вперёд с использованием $gt, без затрат $skip.

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL страницы (очищается на стороне сервера). |
| afterName | string | query | No | Курсор: передайте nextAfterName из предыдущего ответа. (необязательно) |
| afterUserId | string | query | No | Разрывной элемент курсора: передайте nextAfterUserId из предыдущего ответа. Обязательно при задании afterName, чтобы при совпадении имён записи не терялись. (необязательно) |

## Ответ

Возвращает: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_offline_response.go)

## Пример

[inline-code-attrs-start title = 'Пример GetOfflineUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterUserId := "afterUserId_example" // string | Разрывной элемент курсора: передайте nextAfterUserId из предыдущего ответа. Обязательно при задании afterName, чтобы при совпадении имён записи не терялись. (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOfflineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOfflineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// ответ от `GetOfflineUsers`: PageUsersOfflineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOfflineUsers`: %v\n", resp)
}
[inline-code-end]