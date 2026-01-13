## Параметри

| Име | Тип | Място | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Не |  |
| urlId | string | query | Не |  |
| fromCommentId | string | query | Не |  |
| viewed | boolean | query | Не |  |
| type | string | query | Не |  |

## Отговор

Връща: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_notification_count_200_response.go)

## Пример

[inline-code-attrs-start title = 'Пример за GetNotificationCount'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (по избор)
	urlId := "urlId_example" // string |  (по избор)
	fromCommentId := "fromCommentId_example" // string |  (по избор)
	viewed := true // bool |  (по избор)
	type_ := "type__example" // string |  (по избор)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetNotificationCount(context.Background()).TenantId(tenantId).UserId(userId).UrlId(urlId).FromCommentId(fromCommentId).Viewed(viewed).Type_(type_).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetNotificationCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// отговор от `GetNotificationCount`: GetNotificationCount200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetNotificationCount`: %v\n", resp)
}
[inline-code-end]