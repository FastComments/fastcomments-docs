## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_add_hash_tag_200_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα AddHashTag'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string |  (προαιρετικό)
	createHashTagBody := *openapiclient.NewCreateHashTagBody("Tag_example") // CreateHashTagBody |  (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.AddHashTag(context.Background()).TenantId(tenantId).CreateHashTagBody(createHashTagBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.AddHashTag``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απάντηση από `AddHashTag`: AddHashTag200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.AddHashTag`: %v\n", resp)
}
[inline-code-end]

---