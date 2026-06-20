## Parametry

| Nazwa | Typ | Location | Wymagane | Opis |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| approved | boolean | query | No |  |
| sso | string | query | No |  |

## Odpowiedź

Zwraca: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_set_comment_approved_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład PostSetCommentApprovalStatus'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	approved := true // bool |  (opcjonalny)
	sso := "sso_example" // string |  (opcjonalny)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostSetCommentApprovalStatus(context.Background(), commentId).Approved(approved).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostSetCommentApprovalStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `PostSetCommentApprovalStatus`: SetCommentApprovedResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostSetCommentApprovalStatus`: %v\n", resp)
}
[inline-code-end]