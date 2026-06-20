## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| sso | string | query | 否 |  |

## 返回

返回: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_banned_users_count_response.go)

## 示例

[inline-code-attrs-start title = 'GetCounts 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	sso := "sso_example" // string |  (可选)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetCounts(context.Background()).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetCounts``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `GetCounts` 的响应: GetBannedUsersCountResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetCounts`: %v\n", resp)
}
[inline-code-end]