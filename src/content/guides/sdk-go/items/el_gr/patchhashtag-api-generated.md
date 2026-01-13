## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tag | string | path | Ναι |  |
| tenantId | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_patch_hash_tag_200_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα PatchHashTag'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tag := "tag_example" // string | 
	tenantId := "tenantId_example" // string |  (προαιρετικό)
	updateHashTagBody := *openapiclient.NewUpdateHashTagBody() // UpdateHashTagBody |  (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.PatchHashTag(context.Background(), tag).TenantId(tenantId).UpdateHashTagBody(updateHashTagBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.PatchHashTag``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απόκριση από `PatchHashTag`: PatchHashTag200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.PatchHashTag`: %v\n", resp)
}
[inline-code-end]