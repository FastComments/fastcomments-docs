## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|--------------|-------------|------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| direction | string | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Відповідь

Повертає: [`VoteResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_vote_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад PostVote'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	direction := "direction_example" // string |  (опціонально)
	broadcastId := "broadcastId_example" // string |  (опціонально)
	sso := "sso_example" // string |  (опціонально)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostVote(context.Background(), commentId).TenantId(tenantId).Direction(direction).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Помилка при виклику `ModerationAPI.PostVote``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Повна HTTP‑відповідь: %v\n", r)
	}
	// відповідь від `PostVote`: VoteResponse
	fmt.Fprintf(os.Stdout, "Відповідь від `ModerationAPI.PostVote`: %v\n", resp)
}
[inline-code-end]