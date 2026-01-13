## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| commentId | string | query | Ναι |  |
| direction | string | query | Ναι |  |
| userId | string | query | Όχι |  |
| anonUserId | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_vote_comment_200_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα CreateVote'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (προαιρετικό)
	anonUserId := "anonUserId_example" // string |  (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateVote(context.Background()).TenantId(tenantId).CommentId(commentId).Direction(direction).UserId(userId).AnonUserId(anonUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateVote``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απάντηση από το `CreateVote`: VoteComment200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateVote`: %v\n", resp)
}
[inline-code-end]

---