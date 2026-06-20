Ativar ou desativar notificações para um comentário específico.

## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|------------|------------|-----------|
| tenantId | string | query | Yes |  |
| notificationId | string | path | Yes |  |
| optedInOrOut | string | path | Yes |  |
| commentId | string | query | Yes |  |
| sso | string | query | No |  |

## Resposta

Retorna: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_update_user_notification_comment_subscription_status_response.go)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de UpdateUserNotificationCommentSubscriptionStatus'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UpdateUserNotificationCommentSubscriptionStatus(context.Background(), notificationId, optedInOrOut).TenantId(tenantId).CommentId(commentId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UpdateUserNotificationCommentSubscriptionStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// resposta de `UpdateUserNotificationCommentSubscriptionStatus`: UpdateUserNotificationCommentSubscriptionStatusResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UpdateUserNotificationCommentSubscriptionStatus`: %v\n", resp)
}
[inline-code-end]