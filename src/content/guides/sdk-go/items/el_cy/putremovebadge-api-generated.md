## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgeId | string | query | Yes |  |
| userId | string | query | No |  |
| commentId | string | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Απόκριση

Επιστρέφει: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_remove_user_badge_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα PutRemoveBadge'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (προαιρετικό)
	commentId := "commentId_example" // string |  (προαιρετικό)
	broadcastId := "broadcastId_example" // string |  (προαιρετικό)
	sso := "sso_example" // string |  (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PutRemoveBadge(context.Background()).BadgeId(badgeId).UserId(userId).CommentId(commentId).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PutRemoveBadge``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απόκριση από `PutRemoveBadge`: RemoveUserBadgeResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PutRemoveBadge`: %v\n", resp)
}
[inline-code-end]