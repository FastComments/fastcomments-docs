## Parametre

| Navn | Type | Placering | Obligatorisk | Beskrivelse |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| voteId | string | path | Ja |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_vote_delete_response.go)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på DeleteModerationVote'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	voteId := "voteId_example" // string | 
	sso := "sso_example" // string |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.DeleteModerationVote(context.Background(), commentId, voteId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.DeleteModerationVote``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `DeleteModerationVote`: VoteDeleteResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.DeleteModerationVote`: %v\n", resp)
}
[inline-code-end]