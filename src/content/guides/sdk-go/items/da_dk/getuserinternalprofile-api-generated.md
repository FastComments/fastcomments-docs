## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| commentId | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_internal_profile_response.go)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på GetUserInternalProfile'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	commentId := "commentId_example" // string |  (valgfri)
	sso := "sso_example" // string |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetUserInternalProfile(context.Background()).CommentId(commentId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetUserInternalProfile``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `GetUserInternalProfile`: GetUserInternalProfileResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetUserInternalProfile`: %v\n", resp)
}
[inline-code-end]