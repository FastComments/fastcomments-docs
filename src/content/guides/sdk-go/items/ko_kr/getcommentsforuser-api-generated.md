## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | No |  |
| direction | string | query | No |  |
| repliesToUserId | string | query | No |  |
| page | number | query | No |  |
| includei10n | boolean | query | No |  |
| locale | string | query | No |  |
| isCrawler | boolean | query | No |  |

## 반환

Returns: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_for_user_response.go)

## 예제

[inline-code-attrs-start title = 'GetCommentsForUser 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	userId := "userId_example" // string |  (선택 사항)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (선택 사항)
	repliesToUserId := "repliesToUserId_example" // string |  (선택 사항)
	page := float64(1.2) // float64 |  (선택 사항)
	includei10n := true // bool |  (선택 사항)
	locale := "locale_example" // string |  (선택 사항)
	isCrawler := true // bool |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsForUser(context.Background()).UserId(userId).Direction(direction).RepliesToUserId(repliesToUserId).Page(page).Includei10n(includei10n).Locale(locale).IsCrawler(isCrawler).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsForUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetCommentsForUser`의 응답: GetCommentsForUserResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsForUser`: %v\n", resp)
}
[inline-code-end]