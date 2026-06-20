## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαραίτητο | Περιγραφή |
|------|------|----------|----------|-------------|
| badgesUserId | string | query | Όχι |  |
| commentId | string | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_manual_badges_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα GetManualBadgesForUser'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	badgesUserId := "badgesUserId_example" // string |  (προαιρετικό)
	commentId := "commentId_example" // string |  (προαιρετικό)
	sso := "sso_example" // string |  (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetManualBadgesForUser(context.Background()).BadgesUserId(badgesUserId).CommentId(commentId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetManualBadgesForUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απάντηση από `GetManualBadgesForUser`: GetUserManualBadgesResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetManualBadgesForUser`: %v\n", resp)
}
[inline-code-end]

---