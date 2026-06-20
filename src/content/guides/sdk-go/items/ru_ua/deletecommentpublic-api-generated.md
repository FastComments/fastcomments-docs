## Параметры

| Name | Type | Местоположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | путь | Да |  |
| commentId | string | путь | Да |  |
| broadcastId | string | строка запроса | Да |  |
| editKey | string | строка запроса | Нет |  |
| sso | string | строка запроса | Нет |  |

## Ответ

Возвращает: [`PublicAPIDeleteCommentResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_public_api_delete_comment_response.go)

## Пример

[inline-code-attrs-start title = 'Пример DeleteCommentPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	editKey := "editKey_example" // string |  (необязательно)
	sso := "sso_example" // string |  (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.DeleteCommentPublic(context.Background(), tenantId, commentId).BroadcastId(broadcastId).EditKey(editKey).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.DeleteCommentPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// ответ от `DeleteCommentPublic`: PublicAPIDeleteCommentResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.DeleteCommentPublic`: %v\n", resp)
}
[inline-code-end]