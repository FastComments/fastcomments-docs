## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| namespace | string | path | Sì |  |
| component | string | path | Sì |  |
| locale | string | query | No |  |
| useFullTranslationIds | boolean | query | No |  |

## Risposta

Restituisce: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_translations_response.go)

## Esempio

[inline-code-attrs-start title = 'Esempio di GetTranslations'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	locale := "locale_example" // string |  (opzionale)
	useFullTranslationIds := true // bool |  (opzionale)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetTranslations(context.Background(), namespace, component).Locale(locale).UseFullTranslationIds(useFullTranslationIds).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetTranslations``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// risposta da `GetTranslations`: GetTranslationsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetTranslations`: %v\n", resp)
}
[inline-code-end]