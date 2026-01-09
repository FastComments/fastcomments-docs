## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |
| contextUserId | string | query | לא |  |
| isLive | boolean | query | לא |  |

## תגובה

מחזיר: [`DeleteComment200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_delete_comment_200_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-DeleteComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	contextUserId := "contextUserId_example" // string |  (אופציונלי)
	isLive := true // bool |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.DeleteComment(context.Background(), id).TenantId(tenantId).ContextUserId(contextUserId).IsLive(isLive).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.DeleteComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`DeleteComment`: DeleteComment200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.DeleteComment`: %v\n", resp)
}
[inline-code-end]

---