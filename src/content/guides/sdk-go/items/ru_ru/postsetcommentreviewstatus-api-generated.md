## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|--------------|-------------|----------|
| tenantId | string | query | Да |  |
| commentId | string | path | Да |  |
| reviewed | boolean | query | Нет |  |
| broadcastId | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_empty_response.go)

## Пример

[inline-code-attrs-start title = 'Пример PostSetCommentReviewStatus'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // строка | 
	commentId := "commentId_example" // строка | 
	reviewed := true // булево |  (необязательно)
	broadcastId := "broadcastId_example" // строка |  (необязательно)
	sso := "sso_example" // строка |  (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostSetCommentReviewStatus(context.Background(), commentId).TenantId(tenantId).Reviewed(reviewed).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostSetCommentReviewStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// ответ от `PostSetCommentReviewStatus`: APIEmptyResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostSetCommentReviewStatus`: %v\n", resp)
}
[inline-code-end]