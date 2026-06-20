---
## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_suggest_response.go)

## Пример

[inline-code-attrs-start title = 'Пример GetSearchSuggest'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	textSearch := "textSearch_example" // string |  (необязательно)
	sso := "sso_example" // string |  (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetSearchSuggest(context.Background()).TextSearch(textSearch).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetSearchSuggest``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// ответ от `GetSearchSuggest`: ModerationSuggestResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetSearchSuggest`: %v\n", resp)
}
[inline-code-end]

---