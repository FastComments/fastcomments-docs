## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | forespørgsel | Nej |  |

## Respons

Returnerer: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_add_hash_tag_200_response.go)

## Eksempel

[inline-code-attrs-start title = 'AddHashTag Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string |  (valgfrit)
	createHashTagBody := *openapiclient.NewCreateHashTagBody("Tag_example") // CreateHashTagBody |  (valgfrit)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.AddHashTag(context.Background()).TenantId(tenantId).CreateHashTagBody(createHashTagBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.AddHashTag``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `AddHashTag`: AddHashTag200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.AddHashTag`: %v\n", resp)
}
[inline-code-end]

---