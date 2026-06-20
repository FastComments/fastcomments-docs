## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | 否 |  |
| direction | string | query | 否 |  |
| repliesToUserId | string | query | 否 |  |
| page | number | query | 否 |  |
| includei10n | boolean | query | 否 |  |
| locale | string | query | 否 |  |
| isCrawler | boolean | query | 否 |  |

## 回應

回傳: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_for_user_response.go)

## 範例

[inline-code-attrs-start title = 'GetCommentsForUser 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	userId := "userId_example" // string |  (選填)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (選填)
	repliesToUserId := "repliesToUserId_example" // string |  (選填)
	page := float64(1.2) // float64 |  (選填)
	includei10n := true // bool |  (選填)
	locale := "locale_example" // string |  (選填)
	isCrawler := true // bool |  (選填)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsForUser(context.Background()).UserId(userId).Direction(direction).RepliesToUserId(repliesToUserId).Page(page).Includei10n(includei10n).Locale(locale).IsCrawler(isCrawler).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsForUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 從 `GetCommentsForUser` 的回應: GetCommentsForUserResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsForUser`: %v\n", resp)
}
[inline-code-end]