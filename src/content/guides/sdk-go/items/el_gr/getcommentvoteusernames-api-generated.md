## Παράμετροι

| Όνομα | Τύπος | Location | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| commentId | string | path | Ναι |  |
| dir | integer | query | Ναι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comment_vote_user_names_success_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα GetCommentVoteUserNames'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string | 
	dir := int32(56) // int32 | 
	sso := "sso_example" // string |  (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentVoteUserNames(context.Background(), tenantId, commentId).Dir(dir).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentVoteUserNames``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απάντηση από `GetCommentVoteUserNames`: GetCommentVoteUserNamesSuccessResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentVoteUserNames`: %v\n", resp)
}
[inline-code-end]