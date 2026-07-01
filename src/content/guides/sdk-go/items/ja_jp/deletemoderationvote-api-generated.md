## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| voteId | string | path | Yes |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## 応答

Returns: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_vote_delete_response.go)

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
	tenantId := "tenantId_example" // string | 
	commentId := "commentId_example" // string | 
	voteId := "voteId_example" // string | 
	broadcastId := "broadcastId_example" // string |  (オプション)
	sso := "sso_example" // string |  (オプション)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.DeleteModerationVote(context.Background(), commentId, voteId).TenantId(tenantId).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "呼び出し中のエラー `ModerationAPI.DeleteModerationVote``: %v\n", err)
		fmt.Fprintf(os.Stderr, "完全な HTTP 応答: %v\n", r)
	}
	// `DeleteModerationVote` の応答: VoteDeleteResponse
	fmt.Fprintf(os.Stdout, "`ModerationAPI.DeleteModerationVote` からの応答: %v\n", resp)
}
[inline-code-end]

---