## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| commentId | string | path | כן |  |
| editKey | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`GetCommentText200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comment_text_200_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-GetCommentText'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // מחרוזת | 
	commentId := "commentId_example" // מחרוזת | 
	editKey := "editKey_example" // מחרוזת |  (אופציונלי)
	sso := "sso_example" // מחרוזת |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentText(context.Background(), tenantId, commentId).EditKey(editKey).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentText``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`GetCommentText`: GetCommentText200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentText`: %v\n", resp)
}
[inline-code-end]