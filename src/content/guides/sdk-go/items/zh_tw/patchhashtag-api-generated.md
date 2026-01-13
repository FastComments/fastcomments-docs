## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tag | string | 路徑 | 是 |  |
| tenantId | string | 查詢 | 否 |  |

## 回應

回傳: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_patch_hash_tag_200_response.go)

## 範例

[inline-code-attrs-start title = 'PatchHashTag 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tag := "tag_example" // string | 
	tenantId := "tenantId_example" // string |  (可選)
	updateHashTagBody := *openapiclient.NewUpdateHashTagBody() // UpdateHashTagBody |  (可選)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.PatchHashTag(context.Background(), tag).TenantId(tenantId).UpdateHashTagBody(updateHashTagBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.PatchHashTag``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 來自 `PatchHashTag` 的回應: PatchHashTag200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.PatchHashTag`: %v\n", resp)
}
[inline-code-end]