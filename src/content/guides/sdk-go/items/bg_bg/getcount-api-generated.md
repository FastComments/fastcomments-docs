## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | query | Да |  |
| text-search | string | query | Не |  |
| byIPFromComment | string | query | Не |  |
| filter | string | query | Не |  |
| searchFilters | string | query | Не |  |
| demo | boolean | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_count_comments_response.go)

## Пример

[inline-code-attrs-start title = 'GetCount Пример'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	textSearch := "textSearch_example" // string |  (по избор)
	byIPFromComment := "byIPFromComment_example" // string |  (по избор)
	filter := "filter_example" // string |  (по избор)
	searchFilters := "searchFilters_example" // string |  (по избор)
	demo := true // bool |  (по избор)
	sso := "sso_example" // string |  (по избор)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetCount(context.Background()).TenantId(tenantId).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filter(filter).SearchFilters(searchFilters).Demo(demo).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Грешка при извикване `ModerationAPI.GetCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Пълен HTTP отговор: %v\n", r)
	}
	// отговор от `GetCount`: ModerationAPICountCommentsResponse
	fmt.Fprintf(os.Stdout, "Отговор от `ModerationAPI.GetCount`: %v\n", resp)
}
[inline-code-end]