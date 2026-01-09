## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| broadcastId | string | query | Yes |  |
| sso | string | query | No |  |

## Svar

Returnerer: [`LockComment200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_lock_comment_200_response.go)

## Eksempel

[inline-code-attrs-start title = 'LockComment Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.LockComment(context.Background(), tenantId, commentId).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.LockComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `LockComment`: LockComment200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.LockComment`: %v\n", resp)
}
[inline-code-end]