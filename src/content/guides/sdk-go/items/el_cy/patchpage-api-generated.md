## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| id | string | path | Ναι |  |

## Απόκριση

Επιστρέφει: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_patch_page_api_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα PatchPage'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	// απόκριση από το `PatchPage`: PatchPageAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.PatchPage`: %v\n", resp)
}
[inline-code-end]