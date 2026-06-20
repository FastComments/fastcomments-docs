Предишни коментиращи на страницата, които НЕ са в момента онлайн. Подредени по displayName.
Използвайте това след изчерпване на /users/online, за да изобразите секция 'Членове'.
Курсорна пагинация по commenterName: сървърът обхожда частичния индекс {tenantId, urlId, commenterName} от afterName напред чрез $gt, без разход за $skip.

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор на URL адреса на страницата (изчистен от сървъра). |
| afterName | string | query | Не | Курсор: подайте nextAfterName от предишния отговор. |
| afterUserId | string | query | Не | Курсорен критерий за равенство: подайте nextAfterUserId от предишния отговор. Задължително когато afterName е зададен, за да не се изпуснат записи при еднакви имена. |

## Отговор

Връща: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_offline_response.go)

## Пример

[inline-code-attrs-start title = 'Пример за GetOfflineUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Идентификатор на URL адреса на страницата (изчистен от сървъра).
	afterName := "afterName_example" // string | Курсор: подайте nextAfterName от предишния отговор. (по избор)
	afterUserId := "afterUserId_example" // string | Курсорен критерий за равенство: подайте nextAfterUserId от предишния отговор. Задължително когато afterName е зададен, за да не се изпуснат записи при еднакви имена. (по избор)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOfflineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOfflineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// отговор от `GetOfflineUsers`: PageUsersOfflineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOfflineUsers`: %v\n", resp)
}
[inline-code-end]