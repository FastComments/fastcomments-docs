Омогућавање или онемогућавање обавештења за одређени коментар.

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| notificationId | string | path | Да |  |
| optedInOrOut | string | path | Да |  |
| commentId | string | query | Да |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_update_user_notification_comment_subscription_status_response.go)

## Пример

[inline-code-attrs-start title = 'Пример UpdateUserNotificationCommentSubscriptionStatus'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	notificationId := "notificationId_example" // string | 
	optedInOrOut := "optedInOrOut_example" // string | 
	commentId := "commentId_example" // string | 
	sso := "sso_example" // string |  (није обавезно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UpdateUserNotificationCommentSubscriptionStatus(context.Background(), notificationId, optedInOrOut).TenantId(tenantId).CommentId(commentId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UpdateUserNotificationCommentSubscriptionStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// одговор од `UpdateUserNotificationCommentSubscriptionStatus`: UpdateUserNotificationCommentSubscriptionStatusResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UpdateUserNotificationCommentSubscriptionStatus`: %v\n", resp)
}
[inline-code-end]