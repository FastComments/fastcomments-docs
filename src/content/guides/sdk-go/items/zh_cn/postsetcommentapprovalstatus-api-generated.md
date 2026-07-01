## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| commentId | string | path | 是 |  |
| approved | boolean | query | 否 |  |
| broadcastId | string | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回：[`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_set_comment_approved_response.go)

## 示例

[inline-code-attrs-start title = 'PostSetCommentApprovalStatus 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	approved := true // bool |  (可选)
	broadcastId := "broadcastId_example" // string |  (可选)
	sso := "sso_example" // string |  (可选)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostSetCommentApprovalStatus(context.Background(), commentId).TenantId(tenantId).Approved(approved).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "调用 `ModerationAPI.PostSetCommentApprovalStatus`` 时出错：%v\n", err)
		fmt.Fprintf(os.Stderr, "完整 HTTP 响应: %v\n", r)
	}
	// `PostSetCommentApprovalStatus` 的响应: SetCommentApprovedResponse
	fmt.Fprintf(os.Stdout, "来自 `ModerationAPI.PostSetCommentApprovalStatus` 的响应：%v\n", resp)
}
[inline-code-end]