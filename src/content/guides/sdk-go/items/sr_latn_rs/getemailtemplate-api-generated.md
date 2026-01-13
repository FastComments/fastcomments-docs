## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |

## Odgovor

Vraća: [`GetEmailTemplate200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_email_template_200_response.go)

## Primer

[inline-code-attrs-start title = 'Primer GetEmailTemplate'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	// response from `GetEmailTemplate`: GetEmailTemplate200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetEmailTemplate`: %v\n", resp)
}
[inline-code-end]

---