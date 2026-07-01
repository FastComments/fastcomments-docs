## Параметри

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Відповідь

Returns: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_set_comment_text_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад PostSetCommentText'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	setCommentTextParams := *openapiclient.NewSetCommentTextParams("Comment_example") // SetCommentTextParams | 
	broadcastId := "broadcastId_example" // string |  (необов’язково)
	sso := "sso_example" // string |  (необов’язково)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostSetCommentText(context.Background(), commentId).TenantId(tenantId).SetCommentTextParams(setCommentTextParams).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Помилка під час виклику `ModerationAPI.PostSetCommentText``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Повна HTTP відповідь: %v\n", r)
	}
	// відповідь від `PostSetCommentText`: SetCommentTextResponse
	fmt.Fprintf(os.Stdout, "Відповідь від `ModerationAPI.PostSetCommentText`: %v\n", resp)
}
[inline-code-end]