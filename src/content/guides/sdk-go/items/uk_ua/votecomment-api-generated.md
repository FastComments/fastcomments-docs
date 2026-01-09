## Параметри

| Назва | Тип | Розташування | Обов'язкове | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| commentId | string | path | Так |  |
| urlId | string | query | Так |  |
| broadcastId | string | query | Так |  |
| sessionId | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_vote_comment_200_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад VoteComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sessionId := "sessionId_example" // string |  (необов'язково)
	sso := "sso_example" // string |  (необов'язково)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.VoteComment(context.Background(), tenantId, commentId).UrlId(urlId).BroadcastId(broadcastId).VoteBodyParams(voteBodyParams).SessionId(sessionId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.VoteComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь від `VoteComment`: VoteComment200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.VoteComment`: %v\n", resp)
}
[inline-code-end]