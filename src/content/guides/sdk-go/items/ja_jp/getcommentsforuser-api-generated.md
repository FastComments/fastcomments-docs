## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | いいえ |  |
| direction | string | query | いいえ |  |
| repliesToUserId | string | query | いいえ |  |
| page | number | query | いいえ |  |
| includei10n | boolean | query | いいえ |  |
| locale | string | query | いいえ |  |
| isCrawler | boolean | query | いいえ |  |

## レスポンス

戻り値: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_for_user_response.go)

## 例

[inline-code-attrs-start title = 'GetCommentsForUser の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	userId := "userId_example" // string |  (オプション)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (オプション)
	repliesToUserId := "repliesToUserId_example" // string |  (オプション)
	page := float64(1.2) // float64 |  (オプション)
	includei10n := true // bool |  (オプション)
	locale := "locale_example" // string |  (オプション)
	isCrawler := true // bool |  (オプション)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsForUser(context.Background()).UserId(userId).Direction(direction).RepliesToUserId(repliesToUserId).Page(page).Includei10n(includei10n).Locale(locale).IsCrawler(isCrawler).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsForUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetCommentsForUser` のレスポンス: GetCommentsForUserResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsForUser`: %v\n", resp)
}
[inline-code-end]