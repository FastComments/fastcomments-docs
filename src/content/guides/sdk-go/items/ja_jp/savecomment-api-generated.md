## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| isLive | boolean | query | いいえ |  |
| doSpamCheck | boolean | query | いいえ |  |
| sendEmails | boolean | query | いいえ |  |
| populateNotifications | boolean | query | いいえ |  |

## レスポンス

戻り値: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_save_comment_200_response.go)

## 例

[inline-code-attrs-start title = 'SaveComment の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createCommentParams := *openapiclient.NewCreateCommentParams("CommenterName_example", "Comment_example", "Url_example", "UrlId_example", "Locale_example") // CreateCommentParams | 
	isLive := true // bool | （任意）
	doSpamCheck := true // bool | （任意）
	sendEmails := true // bool | （任意）
	populateNotifications := true // bool | （任意）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.SaveComment(context.Background()).TenantId(tenantId).CreateCommentParams(createCommentParams).IsLive(isLive).DoSpamCheck(doSpamCheck).SendEmails(sendEmails).PopulateNotifications(populateNotifications).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.SaveComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `SaveComment`: SaveComment200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.SaveComment`: %v\n", resp)
}
[inline-code-end]

---