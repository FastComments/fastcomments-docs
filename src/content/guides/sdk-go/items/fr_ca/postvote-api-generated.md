## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| commentId | string | chemin | Oui |  |
| direction | string | requête | Non |  |
| sso | string | requête | Non |  |

## Réponse

Retourne: [`VoteResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_vote_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple de PostVote'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	direction := "direction_example" // string |  (optionnel)
	sso := "sso_example" // string |  (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostVote(context.Background(), commentId).Direction(direction).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostVote``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `PostVote`: VoteResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostVote`: %v\n", resp)
}
[inline-code-end]