## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| commentId | string | path | כן |  |
| urlId | string | query | כן |  |
| broadcastId | string | query | כן |  |
| sessionId | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_vote_comment_200_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמת VoteComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	commentId := "commentId_example" // string | 
	urlId := "urlId_example" // string | 
	broadcastId := "broadcastId_example" // string | 
	voteBodyParams := *openapiclient.NewVoteBodyParams("CommenterEmail_example", "CommenterName_example", "VoteDir_example", "Url_example") // VoteBodyParams | 
	sessionId := "sessionId_example" // string |  (אופציונלי)
	sso := "sso_example" // string |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.VoteComment(context.Background(), tenantId, commentId).UrlId(urlId).BroadcastId(broadcastId).VoteBodyParams(voteBodyParams).SessionId(sessionId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.VoteComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ־`VoteComment`: VoteComment200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.VoteComment`: %v\n", resp)
}
[inline-code-end]

---