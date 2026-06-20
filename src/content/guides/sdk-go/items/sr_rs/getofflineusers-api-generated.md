Прошли коментатори на страници који тренутно нису онлајн. Сортирано по displayName.
Користите ово након што исцрпите /users/online да бисте приказали одељак „Members“.
Курсорска пагинација на commenterName: сервер пролази делимични индекс {tenantId, urlId, commenterName} од afterName унапред помоћу $gt, без трошка $skip.

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор URL странице (очишћен на серверу). |
| afterName | string | query | Не | Курсор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | Не | Курсор тајбрекер: проследите nextAfterUserId из претходног одговора. Захтевано када је afterName подешен да уноси са истим именом не буду изостављени. |

## Одговор

Враћа: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_offline_response.go)

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
	urlId := "urlId_example" // string | Page URL identifier (cleaned server-side).
	afterName := "afterName_example" // string | Курсор: проследите nextAfterName из претходног одговора. (опционо)
	afterUserId := "afterUserId_example" // string | Курсор тајбрекер: проследите nextAfterUserId из претходног одговора. Захтевано када је afterName подешен да уноси са истим именом не буду изостављени. (опционо)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOfflineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOfflineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetOfflineUsers`: PageUsersOfflineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOfflineUsers`: %v\n", resp)
}
[inline-code-end]