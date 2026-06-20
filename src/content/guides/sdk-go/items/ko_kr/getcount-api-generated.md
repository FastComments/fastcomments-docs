## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| text-search | string | query | 아니오 |  |
| byIPFromComment | string | query | 아니오 |  |
| filter | string | query | 아니오 |  |
| searchFilters | string | query | 아니오 |  |
| demo | boolean | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_count_comments_response.go)

## 예제

[inline-code-attrs-start title = 'GetCount 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	textSearch := "textSearch_example" // string |  (선택 사항)
	byIPFromComment := "byIPFromComment_example" // string |  (선택 사항)
	filter := "filter_example" // string |  (선택 사항)
	searchFilters := "searchFilters_example" // string |  (선택 사항)
	demo := true // bool |  (선택 사항)
	sso := "sso_example" // string |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetCount(context.Background()).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filter(filter).SearchFilters(searchFilters).Demo(demo).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetCount`의 응답: ModerationAPICountCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetCount`: %v\n", resp)
}
[inline-code-end]

---