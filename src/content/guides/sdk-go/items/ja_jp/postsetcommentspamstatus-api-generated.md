## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | はい |  |
| spam | boolean | query | いいえ |  |
| permNotSpam | boolean | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_empty_response.go)

## 例

[inline-code-attrs-start title = 'PostSetCommentSpamStatus Example'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	commentId := "commentId_example" // string | 
	spam := true // bool |  (任意)
	permNotSpam := true // bool |  (任意)
	sso := "sso_example" // string |  (任意)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostSetCommentSpamStatus(context.Background(), commentId).Spam(spam).PermNotSpam(permNotSpam).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostSetCommentSpamStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `PostSetCommentSpamStatus` からのレスポンス: APIEmptyResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostSetCommentSpamStatus`: %v\n", resp)
}
[inline-code-end]