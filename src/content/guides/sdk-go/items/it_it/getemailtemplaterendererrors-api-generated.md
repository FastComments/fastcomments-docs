## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| id | string | path | Sì |  |
| skip | number | query | No |  |

## Risposta

Restituisce: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_email_template_render_errors_response.go)

## Esempio

[inline-code-attrs-start title = 'Esempio di GetEmailTemplateRenderErrors'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	skip := float64(1.2) // float64 |  (opzionale)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetEmailTemplateRenderErrors(context.Background(), id).TenantId(tenantId).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetEmailTemplateRenderErrors``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// risposta da `GetEmailTemplateRenderErrors`: GetEmailTemplateRenderErrorsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetEmailTemplateRenderErrors`: %v\n", resp)
}
[inline-code-end]