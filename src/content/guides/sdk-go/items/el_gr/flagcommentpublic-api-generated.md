## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| commentId | string | path | Ναι |  |
| isFlagged | boolean | query | Ναι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_flag_comment_public_200_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα FlagCommentPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string | 
	isFlagged := true // bool | 
	sso := "sso_example" // string |  (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.FlagCommentPublic(context.Background(), commentId).TenantId(tenantId).IsFlagged(isFlagged).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.FlagCommentPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απόκριση από `FlagCommentPublic`: FlagCommentPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.FlagCommentPublic`: %v\n", resp)
}
[inline-code-end]