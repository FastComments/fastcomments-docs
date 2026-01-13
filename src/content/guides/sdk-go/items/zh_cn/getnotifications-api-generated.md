## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| userId | string | query | 否 |  |
| urlId | string | query | 否 |  |
| fromCommentId | string | query | 否 |  |
| viewed | boolean | query | 否 |  |
| type | string | query | 否 |  |
| skip | number | query | 否 |  |

## 响应

返回: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_notifications_200_response.go)

## 示例

[inline-code-attrs-start title = 'GetNotifications 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (可选)
	urlId := "urlId_example" // string |  (可选)
	fromCommentId := "fromCommentId_example" // string |  (可选)
	viewed := true // bool |  (可选)
	type_ := "type__example" // string |  (可选)
	skip := float64(1.2) // float64 |  (可选)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetNotifications(context.Background()).TenantId(tenantId).UserId(userId).UrlId(urlId).FromCommentId(fromCommentId).Viewed(viewed).Type_(type_).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetNotifications`: GetNotifications200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetNotifications`: %v\n", resp)
}
[inline-code-end]