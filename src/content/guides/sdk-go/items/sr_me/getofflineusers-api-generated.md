---
Прошли коментатори на страници који тренутно НИСУ онлајн. Сортирано по displayName.
Користите ово након што исцрпите /users/online да бисте приказали одељак "Чланови".
Курсорска пагинација по commenterName: сервер пролази делимични индекс {tenantId, urlId, commenterName}
од afterName напред преко $gt, без трошка $skip.

## Параметри

| Назив | Тип | Локација | Потребно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL странице (очишћен на серверу). |
| afterName | string | query | No | Курсор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | No | Курсор као одлучивач у случају изједначења: проследите nextAfterUserId из претходног одговора. Обавезно када је afterName подешен да имена која се подударају не би изоставила уносе. |

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
	urlId := "urlId_example" // string | Идентификатор URL странице (очишћен на серверу).
	afterName := "afterName_example" // string | Курсор: проследите nextAfterName из претходног одговора. (опционо)
	afterUserId := "afterUserId_example" // string | Курсор као одлучивач у случају изједначења: проследите nextAfterUserId из претходног одговора. Обавезно када је afterName подешен да имена која се подударају не би изоставила уносе. (опционо)

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

---