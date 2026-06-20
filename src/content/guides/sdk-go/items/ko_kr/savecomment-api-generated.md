## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| isLive | boolean | query | 아니요 |  |
| doSpamCheck | boolean | query | 아니요 |  |
| sendEmails | boolean | query | 아니요 |  |
| populateNotifications | boolean | query | 아니요 |  |

## 응답

반환: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_save_comment_response.go)

## 예제

[inline-code-attrs-start title = 'SaveComment 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createCommentParams := *openapiclient.NewCreateCommentParams("CommenterName_example", "Comment_example", "Url_example", "UrlId_example", "Locale_example") // CreateCommentParams | 
	isLive := true // bool |  (선택 사항)
	doSpamCheck := true // bool |  (선택 사항)
	sendEmails := true // bool |  (선택 사항)
	populateNotifications := true // bool |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.SaveComment(context.Background()).TenantId(tenantId).CreateCommentParams(createCommentParams).IsLive(isLive).DoSpamCheck(doSpamCheck).SendEmails(sendEmails).PopulateNotifications(populateNotifications).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.SaveComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `SaveComment`의 응답: APISaveCommentResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.SaveComment`: %v\n", resp)
}
[inline-code-end]