req
tenantId
urlId
userIdWS

## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |
| userIdWS | string | query | Да |  |
| startTime | integer | query | Да |  |
| endTime | integer | query | Да |  |

## Ответ

Возвращает: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_event_log_200_response.go)

## Пример

[inline-code-attrs-start title = 'Пример GetGlobalEventLog'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	urlId := "urlId_example" // string | 
	userIdWS := "userIdWS_example" // string | 
	startTime := int64(789) // int64 | 
	endTime := int64(789) // int64 | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetGlobalEventLog(context.Background(), tenantId).UrlId(urlId).UserIdWS(userIdWS).StartTime(startTime).EndTime(endTime).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetGlobalEventLog``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// ответ от `GetGlobalEventLog`: GetEventLog200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetGlobalEventLog`: %v\n", resp)
}
[inline-code-end]