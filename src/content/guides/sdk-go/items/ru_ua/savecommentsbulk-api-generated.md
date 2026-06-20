## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| isLive | boolean | query | No |  |
| doSpamCheck | boolean | query | No |  |
| sendEmails | boolean | query | No |  |
| populateNotifications | boolean | query | No |  |

## Ответ

Возвращает: [`[]SaveCommentsBulkResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_save_comments_bulk_response.go)

## Пример

[inline-code-attrs-start title = 'Пример SaveCommentsBulk'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createCommentParams := []openapiclient.CreateCommentParams{*openapiclient.NewCreateCommentParams("CommenterName_example", "Comment_example", "Url_example", "UrlId_example", "Locale_example")} // []CreateCommentParams | 
	isLive := true // bool |  (необязательно)
	doSpamCheck := true // bool |  (необязательно)
	sendEmails := true // bool |  (необязательно)
	populateNotifications := true // bool |  (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.SaveCommentsBulk(context.Background()).TenantId(tenantId).CreateCommentParams(createCommentParams).IsLive(isLive).DoSpamCheck(doSpamCheck).SendEmails(sendEmails).PopulateNotifications(populateNotifications).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.SaveCommentsBulk``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// ответ от `SaveCommentsBulk`: []SaveCommentsBulkResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.SaveCommentsBulk`: %v\n", resp)
}
[inline-code-end]