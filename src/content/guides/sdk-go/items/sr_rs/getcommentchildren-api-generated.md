## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## Одговор

Returns: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_child_comments_response.go)

## Пример

[inline-code-attrs-start title = 'Primer GetCommentChildren'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (опционално)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetCommentChildren(context.Background(), commentId).TenantId(tenantId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Грешка при позиву `ModerationAPI.GetCommentChildren``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Пуна HTTP одговор: %v\n", r)
	}
	// одговор од `GetCommentChildren`: ModerationAPIChildCommentsResponse
	fmt.Fprintf(os.Stdout, "Одговор од `ModerationAPI.GetCommentChildren`: %v\n", resp)
}
[inline-code-end]