## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| direction | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`VoteResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_vote_response.go)

## Eksempel

[inline-code-attrs-start title = 'PostVote-eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	direction := "direction_example" // string |  (valgfri)
	sso := "sso_example" // string |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostVote(context.Background(), commentId).Direction(direction).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostVote``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `PostVote`: VoteResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostVote`: %v\n", resp)
}
[inline-code-end]