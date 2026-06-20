## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | yol | Evet |  |
| urlId | string | sorgu | Evet |  |
| title | string | sorgu | Hayır |  |

## Yanıt

Döndürür: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_v1_page_react.go)

## Örnek

[inline-code-attrs-start title = 'CreateV1PageReact Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 
	title := "title_example" // string |  (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.CreateV1PageReact(context.Background(), tenantId).UrlId(urlId).Title(title).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.CreateV1PageReact``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `CreateV1PageReact`'den dönen yanıt: CreateV1PageReact
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.CreateV1PageReact`: %v\n", resp)
}
[inline-code-end]