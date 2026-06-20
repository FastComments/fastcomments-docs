## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_child_comments_response.go)

## Пример

[inline-code-attrs-start title = 'Пример PostCommentsByIds'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	commentsByIdsParams := *openapiclient.NewCommentsByIdsParams([]string{"Ids_example"}) // CommentsByIdsParams | 
	sso := "sso_example" // string |  (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostCommentsByIds(context.Background()).CommentsByIdsParams(commentsByIdsParams).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostCommentsByIds``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// ответ от `PostCommentsByIds`: ModerationAPIChildCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostCommentsByIds`: %v\n", resp)
}
[inline-code-end]