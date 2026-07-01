## Parametre

| Navn | Type | Location | Obligatorisk | Beskrivelse |
|------|------|----------|--------------|-------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Ja |  |
| sso | string | query | Nej |  |

## Svar

Returns: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_empty_response.go)

## Eksempel

[inline-code-attrs-start title = 'PutReopenThread Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PutReopenThread(context.Background()).TenantId(tenantId).UrlId(urlId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Fejl ved kald af `ModerationAPI.PutReopenThread``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Fuld HTTP-respons: %v\n", r)
	}
	// svar fra `PutReopenThread`: APIEmptyResponse
	fmt.Fprintf(os.Stdout, "Svar fra `ModerationAPI.PutReopenThread`: %v\n", resp)
}
[inline-code-end]

---