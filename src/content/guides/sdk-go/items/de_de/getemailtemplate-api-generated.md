---
## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | Abfrage | Ja |  |
| id | string | Pfad | Ja |  |

## Antwort

Gibt zurück: [`GetEmailTemplateResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_email_template_response.go)

## Beispiel

[inline-code-attrs-start title = 'GetEmailTemplate Beispiel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	// Antwort von `GetEmailTemplate`: GetEmailTemplateResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetEmailTemplate`: %v\n", resp)
}
[inline-code-end]

---