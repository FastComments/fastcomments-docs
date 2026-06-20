Массовая информация о пользователях для тенанта. По заданным userIds возвращает отображаемую информацию из User / SSOUser.
Используется виджетом комментариев для дополнения информации о пользователях, которые только что появились через событие присутствия.
Нет контекста страницы: конфиденциальность применяется одинаково (закрытые профили маскируются).

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| ids | string | query | Да | userIds, разделённые запятыми. |

## Ответ

Возвращает: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_info_response.go)

## Пример

[inline-code-attrs-start title = 'Пример GetUsersInfo'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	ids := "ids_example" // string | userIds, разделённые запятыми.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUsersInfo(context.Background(), tenantId).Ids(ids).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUsersInfo``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// ответ от `GetUsersInfo`: PageUsersInfoResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUsersInfo`: %v\n", resp)
}
[inline-code-end]