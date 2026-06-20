## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| badgeId | string | query | Да |  |
| userId | string | query | Нет |  |
| commentId | string | query | Нет |  |
| broadcastId | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_award_user_badge_response.go)

## Пример

[inline-code-attrs-start title = 'Пример PutAwardBadge'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (необязательно)
	commentId := "commentId_example" // string |  (необязательно)
	broadcastId := "broadcastId_example" // string |  (необязательно)
	sso := "sso_example" // string |  (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PutAwardBadge(context.Background()).BadgeId(badgeId).UserId(userId).CommentId(commentId).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PutAwardBadge``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// ответ от `PutAwardBadge`: AwardUserBadgeResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PutAwardBadge`: %v\n", resp)
}
[inline-code-end]