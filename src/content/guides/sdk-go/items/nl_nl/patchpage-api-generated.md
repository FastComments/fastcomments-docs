## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Respons

Retourneert: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_patch_page_api_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'PatchPage Voorbeeld'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	id := "id_example" // string | 
	updateAPIPageData := *openapiclient.NewUpdateAPIPageData() // UpdateAPIPageData | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.PatchPage(context.Background(), id).TenantId(tenantId).UpdateAPIPageData(updateAPIPageData).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.PatchPage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// reactie van `PatchPage`: PatchPageAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.PatchPage`: %v\n", resp)
}
[inline-code-end]