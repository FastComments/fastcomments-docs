## Parameters

| Назва | Тип | Розташування | Обов'язкове | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| commentId | string | path | Так |  |
| broadcastId | string | query | Так |  |
| editKey | string | query | Ні |  |
| sso | string | query | Ні |  |

## Response

Повертає: [`PublicAPIDeleteCommentResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_public_api_delete_comment_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад DeleteCommentPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	broadcastId := "broadcastId_example" // string | 
	editKey := "editKey_example" // string |  (необов'язково)
	sso := "sso_example" // string |  (необов'язково)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.DeleteCommentPublic(context.Background(), tenantId, commentId).BroadcastId(broadcastId).EditKey(editKey).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.DeleteCommentPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь від `DeleteCommentPublic`: PublicAPIDeleteCommentResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.DeleteCommentPublic`: %v\n", resp)
}
[inline-code-end]