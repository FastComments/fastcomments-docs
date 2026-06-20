## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| namespace | string | path | Так |  |
| component | string | path | Так |  |
| locale | string | query | Ні |  |
| useFullTranslationIds | boolean | query | Ні |  |

## Відповідь

Повертає: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_translations_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад GetTranslations'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	locale := "locale_example" // string |  (необов'язково)
	useFullTranslationIds := true // bool |  (необов'язково)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetTranslations(context.Background(), namespace, component).Locale(locale).UseFullTranslationIds(useFullTranslationIds).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetTranslations``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь від `GetTranslations`: GetTranslationsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetTranslations`: %v\n", resp)
}
[inline-code-end]