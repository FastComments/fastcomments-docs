## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| commentId | string | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_internal_profile_response.go)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für GetUserInternalProfile'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	commentId := "commentId_example" // string |  (optional)
	sso := "sso_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetUserInternalProfile(context.Background()).CommentId(commentId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetUserInternalProfile``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `GetUserInternalProfile`: GetUserInternalProfileResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetUserInternalProfile`: %v\n", resp)
}
[inline-code-end]