## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| badgesUserId | string | query | Nee |  |
| commentId | string | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Geeft terug: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_manual_badges_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'GetManualBadgesForUser Voorbeeld'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	badgesUserId := "badgesUserId_example" // string |  (optioneel)
	commentId := "commentId_example" // string |  (optioneel)
	sso := "sso_example" // string |  (optioneel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetManualBadgesForUser(context.Background()).BadgesUserId(badgesUserId).CommentId(commentId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetManualBadgesForUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// antwoord van `GetManualBadgesForUser`: GetUserManualBadgesResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetManualBadgesForUser`: %v\n", resp)
}
[inline-code-end]