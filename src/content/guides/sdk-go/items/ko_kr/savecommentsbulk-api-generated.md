## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| isLive | boolean | query | 아니요 |  |
| doSpamCheck | boolean | query | 아니요 |  |
| sendEmails | boolean | query | 아니요 |  |
| populateNotifications | boolean | query | 아니요 |  |

## 응답

반환: [`[]SaveCommentsBulkResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_save_comments_bulk_response.go)

## 예제

[inline-code-attrs-start title = 'SaveCommentsBulk 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	isLive := true // bool |  (선택 사항)
	doSpamCheck := true // bool |  (선택 사항)
	sendEmails := true // bool |  (선택 사항)
	populateNotifications := true // bool |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.SaveCommentsBulk(context.Background()).TenantId(tenantId).CreateCommentParams(createCommentParams).IsLive(isLive).DoSpamCheck(doSpamCheck).SendEmails(sendEmails).PopulateNotifications(populateNotifications).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.SaveCommentsBulk``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `SaveCommentsBulk`의 응답: []SaveCommentsBulkResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.SaveCommentsBulk`: %v\n", resp)
}
[inline-code-end]