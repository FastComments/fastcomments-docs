## Παράμετροι

| Όνομα | Τύπος | Location | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| commentId | string | path | Ναι |  |
| urlId | string | query | Ναι |  |
| broadcastId | string | query | Ναι |  |
| sessionId | string | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`VoteResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_vote_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα VoteComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 
	broadcastId := "broadcastId_example" // string | 
	voteBodyParams := *openapiclient.NewVoteBodyParams("CommenterEmail_example", "CommenterName_example", "VoteDir_example", "Url_example") // VoteBodyParams | 
	sessionId := "sessionId_example" // string |  (προαιρετικό)
	sso := "sso_example" // string |  (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.VoteComment(context.Background(), tenantId, commentId).UrlId(urlId).BroadcastId(broadcastId).VoteBodyParams(voteBodyParams).SessionId(sessionId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.VoteComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απόκριση από `VoteComment`: VoteResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.VoteComment`: %v\n", resp)
}
[inline-code-end]