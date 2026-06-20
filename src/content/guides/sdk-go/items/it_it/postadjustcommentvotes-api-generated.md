## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| commentId | string | path | Sì |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_adjust_votes_response.go)

## Esempio

[inline-code-attrs-start title = 'Esempio di PostAdjustCommentVotes'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	adjustCommentVotesParams := *openapiclient.NewAdjustCommentVotesParams(float64(123)) // AdjustCommentVotesParams | 
	sso := "sso_example" // string |  (opzionale)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostAdjustCommentVotes(context.Background(), commentId).AdjustCommentVotesParams(adjustCommentVotesParams).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostAdjustCommentVotes``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// risposta da `PostAdjustCommentVotes`: AdjustVotesResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostAdjustCommentVotes`: %v\n", resp)
}
[inline-code-end]