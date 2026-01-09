## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## 响应

返回: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_subscription_api_response.go)

## 示例

[inline-code-attrs-start title = 'CreateSubscription 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	createAPIUserSubscriptionData := *openapiclient.NewCreateAPIUserSubscriptionData("UrlId_example") // CreateAPIUserSubscriptionData | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateSubscription(context.Background()).TenantId(tenantId).CreateAPIUserSubscriptionData(createAPIUserSubscriptionData).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateSubscription``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `CreateSubscription` 的响应: CreateSubscriptionAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateSubscription`: %v\n", resp)
}
[inline-code-end]

---