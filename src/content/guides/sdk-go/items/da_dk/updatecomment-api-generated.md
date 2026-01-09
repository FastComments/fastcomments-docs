## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| contextUserId | string | query | Nej |  |
| doSpamCheck | boolean | query | Nej |  |
| isLive | boolean | query | Nej |  |

## Respons

Returnerer: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_flag_comment_public_200_response.go)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på UpdateComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	id := "id_example" // string | 
	body := PickAPICommentUpdatableCommentFields(987) // PickAPICommentUpdatableCommentFields | 
	contextUserId := "contextUserId_example" // string |  (valgfri)
	doSpamCheck := true // bool |  (valgfri)
	isLive := true // bool |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.UpdateComment(context.Background(), id).TenantId(tenantId).Body(body).ContextUserId(contextUserId).DoSpamCheck(doSpamCheck).IsLive(isLive).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.UpdateComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `UpdateComment`: FlagCommentPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.UpdateComment`: %v\n", resp)
}
[inline-code-end]