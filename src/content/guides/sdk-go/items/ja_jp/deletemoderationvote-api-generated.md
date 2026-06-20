---
## パラメータ

| 名前 | 型 | ロケーション | 必須 | 説明 |
|------|------|----------|----------|-------------|
| commentId | string | path | はい |  |
| voteId | string | path | はい |  |
| sso | string | query | いいえ |  |

## レスポンス

返却値: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_vote_delete_response.go)

## 例

[inline-code-attrs-start title = 'DeleteModerationVote の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	voteId := "voteId_example" // string | 
	sso := "sso_example" // string |  (任意)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.DeleteModerationVote(context.Background(), commentId, voteId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.DeleteModerationVote``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DeleteModerationVote`: VoteDeleteResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.DeleteModerationVote`: %v\n", resp)
}
[inline-code-end]

---