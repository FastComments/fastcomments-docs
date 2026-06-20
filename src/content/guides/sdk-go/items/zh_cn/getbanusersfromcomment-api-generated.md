## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| commentId | string | path | 是 |  |
| sso | string | query | 否 |  |

## 返回

返回：[`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_banned_users_from_comment_response.go)

## 示例

[inline-code-attrs-start title = 'GetBanUsersFromComment 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (可选)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetBanUsersFromComment(context.Background(), commentId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetBanUsersFromComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `GetBanUsersFromComment` 的响应：GetBannedUsersFromCommentResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetBanUsersFromComment`: %v\n", resp)
}
[inline-code-end]