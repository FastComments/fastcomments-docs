## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | sti | Ja |  |
| approved | boolean | forespørgsel | Nej |  |
| sso | string | forespørgsel | Nej |  |

## Svar

Returnerer: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_set_comment_approved_response.go)

## Eksempel

[inline-code-attrs-start title = 'PostSetCommentApprovalStatus Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	commentId := "commentId_example" // string | 
	approved := true // bool |  (valgfri)
	sso := "sso_example" // string |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostSetCommentApprovalStatus(context.Background(), commentId).Approved(approved).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostSetCommentApprovalStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `PostSetCommentApprovalStatus`: SetCommentApprovedResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostSetCommentApprovalStatus`: %v\n", resp)
}
[inline-code-end]

---