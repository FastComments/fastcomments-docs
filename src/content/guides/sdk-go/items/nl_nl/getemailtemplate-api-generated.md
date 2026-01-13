## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Respons

Retourneert: [`GetEmailTemplate200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_email_template_200_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'GetEmailTemplate Voorbeeld'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	id := "id_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetEmailTemplate(context.Background(), id).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetEmailTemplate``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// antwoord van `GetEmailTemplate`: GetEmailTemplate200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetEmailTemplate`: %v\n", resp)
}
[inline-code-end]

---