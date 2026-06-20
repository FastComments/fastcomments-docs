## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgeId | string | query | Да |  |
| userId | string | query | Не |  |
| commentId | string | query | Не |  |
| broadcastId | string | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_award_user_badge_response.go)

## Пример

[inline-code-attrs-start title = 'PutAwardBadge Пример'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (опционо)
	commentId := "commentId_example" // string |  (опционо)
	broadcastId := "broadcastId_example" // string |  (опционо)
	sso := "sso_example" // string |  (опционо)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PutAwardBadge(context.Background()).BadgeId(badgeId).UserId(userId).CommentId(commentId).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PutAwardBadge``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// одговор од `PutAwardBadge`: AwardUserBadgeResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PutAwardBadge`: %v\n", resp)
}
[inline-code-end]