## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| badgeId | string | query | Ja |  |
| userId | string | query | Nein |  |
| commentId | string | query | Nein |  |
| broadcastId | string | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_remove_user_badge_response.go)

## Beispiel

[inline-code-attrs-start title = 'PutRemoveBadge Beispiel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	badgeId := "badgeId_example" // string | 
	userId := "userId_example" // string |  (optional)
	commentId := "commentId_example" // string |  (optional)
	broadcastId := "broadcastId_example" // string |  (optional)
	sso := "sso_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PutRemoveBadge(context.Background()).BadgeId(badgeId).UserId(userId).CommentId(commentId).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PutRemoveBadge``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `PutRemoveBadge`: RemoveUserBadgeResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PutRemoveBadge`: %v\n", resp)
}
[inline-code-end]

---