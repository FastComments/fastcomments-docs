## パラメータ

| 名前 | 型 | 位置 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tag | string | path | はい |  |
| tenantId | string | query | いいえ |  |

## レスポンス

戻り値: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_update_hash_tag_response.go)

## 例

[inline-code-attrs-start title = 'PatchHashTag の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	tenantId := "tenantId_example" // string |  （オプション）
	updateHashTagBody := *openapiclient.NewUpdateHashTagBody() // UpdateHashTagBody |  （オプション）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.PatchHashTag(context.Background(), tag).TenantId(tenantId).UpdateHashTagBody(updateHashTagBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.PatchHashTag``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `PatchHashTag` からのレスポンス: UpdateHashTagResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.PatchHashTag`: %v\n", resp)
}
[inline-code-end]

---