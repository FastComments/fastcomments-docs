## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι |  |

## Απόκριση

Επιστρέφει: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_v1_page_react.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα DeleteV1PageReact'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.DeleteV1PageReact(context.Background(), tenantId).UrlId(urlId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.DeleteV1PageReact``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απάντηση από `DeleteV1PageReact`: CreateV1PageReact
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.DeleteV1PageReact`: %v\n", resp)
}
[inline-code-end]