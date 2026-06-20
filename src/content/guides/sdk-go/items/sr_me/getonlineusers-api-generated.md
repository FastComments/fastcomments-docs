Тренутно онлајн гледаоци странице: особе чије websocket сесије су тренутно претплаћене на страницу.
Враћа anonCount + totalCount (сви претплатници у соби, укључујући анонимне гледаоце које не набрајамо).

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | путања | Да |  |
| urlId | string | упит | Да | Идентификатор URL странице (очишћен на серверској страни). |
| afterName | string | упит | Не | Курсор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | упит | Не | Курсор за разликовање: проследите nextAfterUserId из претходног одговора. Потребно када је afterName подешен да би уноси са истим именом остали укључени. |

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
	urlId := "urlId_example" // string | Идентификатор URL странице (очишћен на серверској страни).
	afterName := "afterName_example" // string | Курсор: проследите nextAfterName из претходног одговора. (опционо)
	afterUserId := "afterUserId_example" // string | Курсор за разликовање: проследите nextAfterUserId из претходног одговора. Потребно када је afterName подешен да би уноси са истим именом остали укључени. (опционо)

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

---