## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| page | number | query | 아니오 |  |
| count | number | query | 아니오 |  |
| text-search | string | query | 아니오 |  |
| byIPFromComment | string | query | 아니오 |  |
| filters | string | query | 아니오 |  |
| searchFilters | string | query | 아니오 |  |
| sorts | string | query | 아니오 |  |
| demo | boolean | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_get_comments_response.go)

## 예제

[inline-code-attrs-start title = 'GetApiComments 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	page := float64(1.2) // float64 |  (선택 사항)
	count := float64(1.2) // float64 |  (선택 사항)
	textSearch := "textSearch_example" // string |  (선택 사항)
	byIPFromComment := "byIPFromComment_example" // string |  (선택 사항)
	filters := "filters_example" // string |  (선택 사항)
	searchFilters := "searchFilters_example" // string |  (선택 사항)
	sorts := "sorts_example" // string |  (선택 사항)
	demo := true // bool |  (선택 사항)
	sso := "sso_example" // string |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetApiComments(context.Background()).Page(page).Count(count).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filters(filters).SearchFilters(searchFilters).Sorts(sorts).Demo(demo).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetApiComments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetApiComments`의 응답: ModerationAPIGetCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetApiComments`: %v\n", resp)
}
[inline-code-end]

---