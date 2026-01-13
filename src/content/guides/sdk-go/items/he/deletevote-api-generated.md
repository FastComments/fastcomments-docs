---
## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |
| editKey | string | query | לא |  |

## תגובה

מחזיר: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_delete_comment_vote_200_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה עבור DeleteVote'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	editKey := "editKey_example" // string |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.DeleteVote(context.Background(), id).TenantId(tenantId).EditKey(editKey).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.DeleteVote``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`DeleteVote`: DeleteCommentVote200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.DeleteVote`: %v\n", resp)
}
[inline-code-end]

---