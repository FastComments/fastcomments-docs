## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | 아니오 |  |
| byIPFromComment | string | query | 아니오 |  |
| filters | string | query | 아니오 |  |
| searchFilters | string | query | 아니오 |  |
| sorts | string | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_export_response.go)

## 예제

[inline-code-attrs-start title = 'PostApiExport 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	filters := "filters_example" // string |  (선택 사항)
	searchFilters := "searchFilters_example" // string |  (선택 사항)
	sorts := "sorts_example" // string |  (선택 사항)
	sso := "sso_example" // string |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostApiExport(context.Background()).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filters(filters).SearchFilters(searchFilters).Sorts(sorts).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostApiExport``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `PostApiExport`의 응답: ModerationExportResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostApiExport`: %v\n", resp)
}
[inline-code-end]