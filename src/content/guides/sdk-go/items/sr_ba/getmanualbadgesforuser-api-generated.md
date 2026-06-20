## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| badgesUserId | string | query | No |  |
| commentId | string | query | No |  |
| sso | string | query | No |  |

## Odgovor

Vraća: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_manual_badges_response.go)

## Primjer

[inline-code-attrs-start title = 'Primjer za GetManualBadgesForUser'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	badgesUserId := "badgesUserId_example" // string |  (opcionalno)
	commentId := "commentId_example" // string |  (opcionalno)
	sso := "sso_example" // string |  (opcionalno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetManualBadgesForUser(context.Background()).BadgesUserId(badgesUserId).CommentId(commentId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetManualBadgesForUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor od `GetManualBadgesForUser`: GetUserManualBadgesResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetManualBadgesForUser`: %v\n", resp)
}
[inline-code-end]