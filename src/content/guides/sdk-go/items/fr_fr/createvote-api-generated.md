## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| commentId | string | query | Oui |  |
| direction | string | query | Oui |  |
| userId | string | query | Non |  |
| anonUserId | string | query | Non |  |

## Réponse

Renvoie: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_vote_comment_200_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple CreateVote'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	direction := "direction_example" // string | 
	userId := "userId_example" // string |  (optionnel)
	anonUserId := "anonUserId_example" // string |  (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateVote(context.Background()).TenantId(tenantId).CommentId(commentId).Direction(direction).UserId(userId).AnonUserId(anonUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateVote``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `CreateVote` : VoteComment200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateVote`: %v\n", resp)
}
[inline-code-end]