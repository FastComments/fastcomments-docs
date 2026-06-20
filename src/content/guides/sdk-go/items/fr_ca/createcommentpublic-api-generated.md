## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | chemin | Oui |  |
| urlId | string | requête | Oui |  |
| broadcastId | string | requête | Oui |  |
| sessionId | string | requête | Non |  |
| sso | string | requête | Non |  |

## Réponse

Renvoie: [`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_save_comments_response_with_presence.go)

## Exemple

[inline-code-attrs-start title = 'Exemple CreateCommentPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 
	broadcastId := "broadcastId_example" // string | 
	commentData := *openapiclient.NewCommentData("CommenterName_example", "Comment_example", "Url_example", "UrlId_example") // CommentData | 
	sessionId := "sessionId_example" // string |  (optionnel)
	sso := "sso_example" // string |  (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.CreateCommentPublic(context.Background(), tenantId).UrlId(urlId).BroadcastId(broadcastId).CommentData(commentData).SessionId(sessionId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.CreateCommentPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `CreateCommentPublic`: SaveCommentsResponseWithPresence
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.CreateCommentPublic`: %v\n", resp)
}
[inline-code-end]