## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| commentId | string | path | Ja |  |
| editKey | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`GetCommentText200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comment_text_200_response.go)

## Eksempel

[inline-code-attrs-start title = 'GetCommentText-eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	editKey := "editKey_example" // string |  (valgfri)
	sso := "sso_example" // string |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentText(context.Background(), tenantId, commentId).EditKey(editKey).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentText``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `GetCommentText`: GetCommentText200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentText`: %v\n", resp)
}
[inline-code-end]