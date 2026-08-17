## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | path | Да |  |
| broadcastId | string | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_set_comment_text_response.go)

## Пример

[inline-code-attrs-start title = 'PostSetCommentText пример'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	broadcastId := "broadcastId_example" // string |  (опционално)
	sso := "sso_example" // string |  (опционално)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostSetCommentText(context.Background(), commentId).TenantId(tenantId).SetCommentTextParams(setCommentTextParams).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostSetCommentText``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// одговор од `PostSetCommentText`: SetCommentTextResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostSetCommentText`: %v\n", resp)
}
[inline-code-end]