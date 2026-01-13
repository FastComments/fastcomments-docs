---
## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| commentId | string | query | Sì |  |
| direction | string | query | Sì |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |

## Risposta

Restituisce: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_vote_comment_200_response.go)

## Esempio

[inline-code-attrs-start title = 'Esempio di CreateVote'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (opzionale)
	anonUserId := "anonUserId_example" // string |  (opzionale)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateVote(context.Background()).TenantId(tenantId).CommentId(commentId).Direction(direction).UserId(userId).AnonUserId(anonUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateVote``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// risposta da `CreateVote`: VoteComment200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateVote`: %v\n", resp)
}
[inline-code-end]

---