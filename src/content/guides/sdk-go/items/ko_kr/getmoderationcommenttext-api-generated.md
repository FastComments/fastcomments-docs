## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | 예 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comment_text_response.go)

## 예제

[inline-code-attrs-start title = 'GetModerationCommentText 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetModerationCommentText(context.Background(), commentId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetModerationCommentText``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetModerationCommentText`의 응답: GetCommentTextResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetModerationCommentText`: %v\n", resp)
}
[inline-code-end]

---