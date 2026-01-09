## Paramètres

| Nom | Type | Location | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |

## Réponse

Retourne: [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_add_page_api_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple AddPage'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	createAPIPageData := *openapiclient.NewCreateAPIPageData("Title_example", "Url_example", "UrlId_example") // CreateAPIPageData | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.AddPage(context.Background()).TenantId(tenantId).CreateAPIPageData(createAPIPageData).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.AddPage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `AddPage`: AddPageAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.AddPage`: %v\n", resp)
}
[inline-code-end]