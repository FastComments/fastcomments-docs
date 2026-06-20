Тренутно онлајн посматрачи странице: људи чија websocket сесија је управо претплаћена на страницу.
Враћа anonCount + totalCount (претплатници по просторији, укључујући анонимне гледаоце које не набрајамо).

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор URL странице (очишћен на серверу). |
| afterName | string | query | Не | Курсор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | Не | Курсор за разликовање: проследите nextAfterUserId из претходног одговора. Обавезно када је afterName постављен да се не би изгубиле ставке због истих имена. |

## Одговор

Враћа: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_online_response.go)

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
	urlId := "urlId_example" // string | Идентификатор URL странице (очишћен на серверу).
	afterName := "afterName_example" // string | Курсор: проследите nextAfterName из претходног одговора. (опционо)
	afterUserId := "afterUserId_example" // string | Курсор за разликовање: проследите nextAfterUserId из претходног одговора. Обавезно када је afterName постављен да се не би изгубиле ставке због истих имена. (опционо)

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