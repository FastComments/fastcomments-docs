## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|-----|-----|---------------|--------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Ja |  |
| sso | string | query | Nein |  |

## Antwort

Rückgabe: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_empty_response.go)

## Beispiel

[inline-code-attrs-start title = 'PutCloseThread Beispiel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PutCloseThread(context.Background()).TenantId(tenantId).UrlId(urlId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Fehler beim Aufruf von `ModerationAPI.PutCloseThread``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Vollständige HTTP-Antwort: %v\n", r)
	}
	// Antwort von `PutCloseThread`: APIEmptyResponse
	fmt.Fprintf(os.Stdout, "Antwort von `ModerationAPI.PutCloseThread`: %v\n", resp)
}
[inline-code-end]