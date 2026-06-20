## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| includeByUserIdAndEmail | boolean | query | 否 |  |
| includeByIP | boolean | query | 否 |  |
| includeByEmailDomain | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_bulk_pre_ban_summary.go)

## 示例

[inline-code-attrs-start title = 'PostBulkPreBanSummary 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	bulkPreBanParams := *openapiclient.NewBulkPreBanParams([]string{"CommentIds_example"}) // BulkPreBanParams | 
	includeByUserIdAndEmail := true // bool |  (可选)
	includeByIP := true // bool |  (可选)
	includeByEmailDomain := true // bool |  (可选)
	sso := "sso_example" // string |  (可选)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostBulkPreBanSummary(context.Background()).BulkPreBanParams(bulkPreBanParams).IncludeByUserIdAndEmail(includeByUserIdAndEmail).IncludeByIP(includeByIP).IncludeByEmailDomain(includeByEmailDomain).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostBulkPreBanSummary``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `PostBulkPreBanSummary` 的响应：BulkPreBanSummary
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostBulkPreBanSummary`: %v\n", resp)
}
[inline-code-end]