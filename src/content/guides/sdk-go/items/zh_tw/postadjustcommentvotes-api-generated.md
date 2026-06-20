## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| commentId | string | path | 是 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_adjust_votes_response.go)

## 範例

[inline-code-attrs-start title = 'PostAdjustCommentVotes 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	adjustCommentVotesParams := *openapiclient.NewAdjustCommentVotesParams(float64(123)) // AdjustCommentVotesParams | 
	sso := "sso_example" // string |  (可選)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostAdjustCommentVotes(context.Background(), commentId).AdjustCommentVotesParams(adjustCommentVotesParams).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostAdjustCommentVotes``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `PostAdjustCommentVotes` 的回應: AdjustVotesResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostAdjustCommentVotes`: %v\n", resp)
}
[inline-code-end]