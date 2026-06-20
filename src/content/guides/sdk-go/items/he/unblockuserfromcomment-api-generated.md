## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |
| userId | string | query | לא |  |
| anonUserId | string | query | לא |  |

## תגובה

מחזיר: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_unblock_success.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמת UnBlockUserFromComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	id := "id_example" // string | 
	unBlockFromCommentParams := *openapiclient.NewUnBlockFromCommentParams() // UnBlockFromCommentParams | 
	userId := "userId_example" // string |  (אופציונלי)
	anonUserId := "anonUserId_example" // string |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.UnBlockUserFromComment(context.Background(), id).TenantId(tenantId).UnBlockFromCommentParams(unBlockFromCommentParams).UserId(userId).AnonUserId(anonUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.UnBlockUserFromComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ`UnBlockUserFromComment`: UnblockSuccess
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.UnBlockUserFromComment`: %v\n", resp)
}
[inline-code-end]