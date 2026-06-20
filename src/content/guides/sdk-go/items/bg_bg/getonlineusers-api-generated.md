В момента онлайн зрители на страница: хора, чиито websocket сесии са абонирани за страницата в момента.
Връща anonCount + totalCount (абонати в рамките на стаята, включително анонимни зрители, които не изброяваме).

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор на URL на страницата (почистен от страна на сървъра). |
| afterName | string | query | Не | Курсор: подайте nextAfterName от предишния отговор. |
| afterUserId | string | query | Не | Курсор за разрешаване на равенство: подайте nextAfterUserId от предишния отговор. Задължително когато afterName е зададено, за да не се загубят записи при еднакви имена. |

## Отговор

Връща: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_online_response.go)

## Пример

[inline-code-attrs-start title = 'Пример за GetOnlineUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Идентификатор на URL на страницата (почистен от страна на сървъра).
	afterName := "afterName_example" // string | Курсор: подайте nextAfterName от предишния отговор. (по избор)
	afterUserId := "afterUserId_example" // string | Курсор за разрешаване на равенство: подайте nextAfterUserId от предишния отговор. Задължително когато afterName е зададено, за да не се изпуснат записи при еднакви имена. (по избор)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOnlineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOnlineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// отговор от `GetOnlineUsers`: PageUsersOnlineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOnlineUsers`: %v\n", resp)
}
[inline-code-end]