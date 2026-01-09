## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| urlId | string | query | Tak |  |
| broadcastId | string | query | Tak |  |
| sessionId | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`CreateCommentPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_comment_public_200_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład CreateCommentPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 
	broadcastId := "broadcastId_example" // string | 
	commentData := *openapiclient.NewCommentData("CommenterName_example", "Comment_example", "Url_example", "UrlId_example") // CommentData | 
	sessionId := "sessionId_example" // string |  (opcjonalne)
	sso := "sso_example" // string |  (opcjonalne)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.CreateCommentPublic(context.Background(), tenantId).UrlId(urlId).BroadcastId(broadcastId).CommentData(commentData).SessionId(sessionId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.CreateCommentPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `CreateCommentPublic`: CreateCommentPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.CreateCommentPublic`: %v\n", resp)
}
[inline-code-end]