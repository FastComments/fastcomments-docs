---
## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| id | string | path | Ναι |  |
| contextUserId | string | query | Όχι |  |
| doSpamCheck | boolean | query | Όχι |  |
| isLive | boolean | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_flag_comment_public_200_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα UpdateComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	contextUserId := "contextUserId_example" // string |  (προαιρετικό)
	doSpamCheck := true // bool |  (προαιρετικό)
	isLive := true // bool |  (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.UpdateComment(context.Background(), id).TenantId(tenantId).Body(body).ContextUserId(contextUserId).DoSpamCheck(doSpamCheck).IsLive(isLive).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.UpdateComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UpdateComment`: FlagCommentPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.UpdateComment`: %v\n", resp)
}
[inline-code-end]

---