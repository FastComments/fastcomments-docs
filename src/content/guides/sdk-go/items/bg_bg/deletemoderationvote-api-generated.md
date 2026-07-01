## Параметри

| Име | Тип | Location | Задължително | Описание |
|------|------|----------|--------------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | path | Да |  |
| voteId | string | path | Да |  |
| broadcastId | string | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_vote_delete_response.go)

## Пример

[inline-code-attrs-start title = 'Пример за DeleteModerationVote'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	voteId := "voteId_example" // string | 
	broadcastId := "broadcastId_example" // string |  (по избор)
	sso := "sso_example" // string |  (по избор)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.DeleteModerationVote(context.Background(), commentId, voteId).TenantId(tenantId).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Грешка при извикване на `ModerationAPI.DeleteModerationVote``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Пълен HTTP отговор: %v\n", r)
	}
	// отговор от `DeleteModerationVote`: VoteDeleteResponse
	fmt.Fprintf(os.Stdout, "Отговор от `ModerationAPI.DeleteModerationVote`: %v\n", resp)
}
[inline-code-end]