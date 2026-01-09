## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| commentId | string | path | Да |  |
| broadcastId | string | query | Да |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`LockComment200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_lock_comment_200_response.go)

## Пример

[inline-code-attrs-start title = 'Пример UnLockComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	commentId := "commentId_example" // string | 
	broadcastId := "broadcastId_example" // string | 
	sso := "sso_example" // string |  (опционо)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UnLockComment(context.Background(), tenantId, commentId).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UnLockComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// одговор од `UnLockComment`: LockComment200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UnLockComment`: %v\n", resp)
}
[inline-code-end]