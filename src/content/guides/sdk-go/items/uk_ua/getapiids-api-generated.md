## Параметри

| Назва | Тип | Розташування | Обов'язкове | Опис |
|------|------|----------|----------|-------------|
| text-search | string | query | Ні |  |
| byIPFromComment | string | query | Ні |  |
| filters | string | query | Ні |  |
| searchFilters | string | query | Ні |  |
| afterId | string | query | Ні |  |
| demo | boolean | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_get_comment_ids_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад GetApiIds'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	textSearch := "textSearch_example" // string |  (необов'язково)
	byIPFromComment := "byIPFromComment_example" // string |  (необов'язково)
	filters := "filters_example" // string |  (необов'язково)
	searchFilters := "searchFilters_example" // string |  (необов'язково)
	afterId := "afterId_example" // string |  (необов'язково)
	demo := true // bool |  (необов'язково)
	sso := "sso_example" // string |  (необов'язково)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetApiIds(context.Background()).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filters(filters).SearchFilters(searchFilters).AfterId(afterId).Demo(demo).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetApiIds``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь від `GetApiIds`: ModerationAPIGetCommentIdsResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetApiIds`: %v\n", resp)
}
[inline-code-end]

---