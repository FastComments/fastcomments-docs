## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| badgesUserId | string | query | לא |  |
| commentId | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_manual_badges_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה עבור GetManualBadgesForUser'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	badgesUserId := "badgesUserId_example" // string | (אופציונלי)
	commentId := "commentId_example" // string | (אופציונלי)
	sso := "sso_example" // string | (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetManualBadgesForUser(context.Background()).BadgesUserId(badgesUserId).CommentId(commentId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetManualBadgesForUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תשובה מ-`GetManualBadgesForUser`: GetUserManualBadgesResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetManualBadgesForUser`: %v\n", resp)
}
[inline-code-end]