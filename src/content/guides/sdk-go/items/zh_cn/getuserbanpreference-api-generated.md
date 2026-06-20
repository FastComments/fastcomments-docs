---
## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| sso | string | query | 否 |  |

## 响应

返回: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_moderate_get_user_ban_preferences_response.go)

## 示例

[inline-code-attrs-start title = 'GetUserBanPreference 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	resp, r, err := apiClient.ModerationAPI.GetUserBanPreference(context.Background()).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetUserBanPreference``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `GetUserBanPreference` 的响应: APIModerateGetUserBanPreferencesResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetUserBanPreference`: %v\n", resp)
}
[inline-code-end]

---