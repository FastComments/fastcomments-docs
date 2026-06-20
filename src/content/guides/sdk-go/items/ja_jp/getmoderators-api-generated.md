## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | クエリ | はい |  |
| skip | number | クエリ | いいえ |  |

## レスポンス

戻り値: [`GetModeratorsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_moderators_response.go)

## 例

[inline-code-attrs-start title = 'GetModerators の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	skip := float64(1.2) // float64 |  (オプション)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetModerators(context.Background()).TenantId(tenantId).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetModerators``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetModerators` のレスポンス: GetModeratorsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetModerators`: %v\n", resp)
}
[inline-code-end]

---