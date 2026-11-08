## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| namespace | string | path | हाँ |  |
| component | string | path | हाँ |  |
| locale | string | query | नहीं |  |
| useFullTranslationIds | boolean | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_translations_response.go)

## उदाहरण

[inline-code-attrs-start title = 'GetTranslations उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	locale := "locale_example" // string |  (वैकल्पिक)
	useFullTranslationIds := true // bool |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetTranslations(context.Background(), namespace, component).Locale(locale).UseFullTranslationIds(useFullTranslationIds).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetTranslations``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetTranslations`: GetTranslationsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetTranslations`: %v\n", resp)
}
[inline-code-end]