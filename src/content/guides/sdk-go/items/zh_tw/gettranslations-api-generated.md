## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| namespace | string | path | 是 |  |
| component | string | path | 是 |  |
| locale | string | query | 否 |  |
| useFullTranslationIds | boolean | query | 否 |  |

## 回應

回傳: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_translations_response.go)

## 範例

[inline-code-attrs-start title = 'GetTranslations 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	namespace := "namespace_example" // string | 
	component := "component_example" // string | 
	locale := "locale_example" // string |  (可選)
	useFullTranslationIds := true // bool |  (可選)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetTranslations(context.Background(), namespace, component).Locale(locale).UseFullTranslationIds(useFullTranslationIds).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetTranslations``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 從 `GetTranslations` 的回應： GetTranslationsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetTranslations`: %v\n", resp)
}
[inline-code-end]