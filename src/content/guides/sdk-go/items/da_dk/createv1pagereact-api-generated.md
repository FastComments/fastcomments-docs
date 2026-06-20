## Parametre

| Name | Type | Location | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| title | string | query | Nej |  |

## Svar

Returnerer: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_v1_page_react.go)

## Eksempel

[inline-code-attrs-start title = 'CreateV1PageReact Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	title := "title_example" // string |  (valgfrit)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.CreateV1PageReact(context.Background(), tenantId).UrlId(urlId).Title(title).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.CreateV1PageReact``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `CreateV1PageReact`: CreateV1PageReact
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.CreateV1PageReact`: %v\n", resp)
}
[inline-code-end]

---