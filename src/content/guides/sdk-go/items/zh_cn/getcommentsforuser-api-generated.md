---
## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | No |  |
| direction | string | query | No |  |
| repliesToUserId | string | query | No |  |
| page | number | query | No |  |
| includei10n | boolean | query | No |  |
| locale | string | query | No |  |
| isCrawler | boolean | query | No |  |

## 响应

返回: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_for_user_response.go)

## 示例

[inline-code-attrs-start title = 'GetCommentsForUser 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	userId := "userId_example" // string |  (可选)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (可选)
	repliesToUserId := "repliesToUserId_example" // string |  (可选)
	page := float64(1.2) // float64 |  (可选)
	includei10n := true // bool |  (可选)
	locale := "locale_example" // string |  (可选)
	isCrawler := true // bool |  (可选)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsForUser(context.Background()).UserId(userId).Direction(direction).RepliesToUserId(repliesToUserId).Page(page).Includei10n(includei10n).Locale(locale).IsCrawler(isCrawler).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsForUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `GetCommentsForUser` 的响应: GetCommentsForUserResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsForUser`: %v\n", resp)
}
[inline-code-end]

---