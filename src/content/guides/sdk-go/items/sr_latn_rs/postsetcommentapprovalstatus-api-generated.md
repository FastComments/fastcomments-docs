## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| commentId | string | path | Da |  |
| approved | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_set_comment_approved_response.go)

## Primer

[inline-code-attrs-start title = 'Primer PostSetCommentApprovalStatus'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	approved := true // bool |  (neobavezno)
	sso := "sso_example" // string |  (neobavezno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostSetCommentApprovalStatus(context.Background(), commentId).Approved(approved).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostSetCommentApprovalStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor od `PostSetCommentApprovalStatus`: SetCommentApprovedResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostSetCommentApprovalStatus`: %v\n", resp)
}
[inline-code-end]