## Параметри

| Назва | Тип | Розташування | Обов’язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| sorts | string | query | No |  |
| sso | string | query | No |  |

## Відповідь

Returns: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_export_response.go)

## Приклад

[inline-code-attrs-start title = 'PostApiExport Приклад'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // рядок |
	textSearch := "textSearch_example" // рядок |  (необов’язково)
	byIPFromComment := "byIPFromComment_example" // рядок |  (необов’язково)
	filters := "filters_example" // рядок |  (необов’язково)
	searchFilters := "searchFilters_example" // рядок |  (необов’язково)
	sorts := "sorts_example" // рядок |  (необов’язково)
	sso := "sso_example" // рядок |  (необов’язково)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostApiExport(context.Background()).TenantId(tenantId).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filters(filters).SearchFilters(searchFilters).Sorts(sorts).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Помилка під час виклику `ModerationAPI.PostApiExport``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Повна HTTP відповідь: %v\n", r)
	}
	// відповідь від `PostApiExport`: ModerationExportResponse
	fmt.Fprintf(os.Stdout, "Відповідь від `ModerationAPI.PostApiExport`: %v\n", resp)
}
[inline-code-end]