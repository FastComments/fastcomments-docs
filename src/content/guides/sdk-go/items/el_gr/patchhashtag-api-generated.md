## Παράμετροι

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| tag | string | path | Yes |  |

## Απάντηση

Επιστρέφει: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_update_hash_tag_response.go)

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
	tenantId := "tenantId_example" // string | 
	tag := "tag_example" // string | 
	updateHashTagBody := *openapiclient.NewUpdateHashTagBody() // UpdateHashTagBody | (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.PatchHashTag(context.Background(), tag).TenantId(tenantId).UpdateHashTagBody(updateHashTagBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Σφάλμα κατά την κλήση `DefaultAPI.PatchHashTag``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Πλήρης απόκριση HTTP: %v\n", r)
	}
	// απάντηση από `PatchHashTag`: UpdateHashTagResponse
	fmt.Fprintf(os.Stdout, "Απάντηση από `DefaultAPI.PatchHashTag`: %v\n", resp)
}
[inline-code-end]