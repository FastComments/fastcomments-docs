## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| value | string | query | 아니요 |  |
| filters | string | query | 아니요 |  |
| searchFilters | string | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_comment_search_response.go)

## 예제

[inline-code-attrs-start title = 'GetSearchCommentsSummary 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	value := "value_example" // string |  (선택사항)
	filters := "filters_example" // string |  (선택사항)
	searchFilters := "searchFilters_example" // string |  (선택사항)
	sso := "sso_example" // string |  (선택사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetSearchCommentsSummary(context.Background()).Value(value).Filters(filters).SearchFilters(searchFilters).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetSearchCommentsSummary``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetSearchCommentsSummary`의 응답: ModerationCommentSearchResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetSearchCommentsSummary`: %v\n", resp)
}
[inline-code-end]