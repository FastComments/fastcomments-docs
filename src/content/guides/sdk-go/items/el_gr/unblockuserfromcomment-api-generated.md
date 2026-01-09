## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| id | string | path | Ναι |  |
| userId | string | query | Όχι |  |
| anonUserId | string | query | Όχι |  |

## Απάντηση

Επιστρέφει: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_un_block_comment_public_200_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα UnBlockUserFromComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	unBlockFromCommentParams := *openapiclient.NewUnBlockFromCommentParams() // UnBlockFromCommentParams | 
	userId := "userId_example" // string |  (προαιρετικό)
	anonUserId := "anonUserId_example" // string |  (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.UnBlockUserFromComment(context.Background(), id).TenantId(tenantId).UnBlockFromCommentParams(unBlockFromCommentParams).UserId(userId).AnonUserId(anonUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.UnBlockUserFromComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απόκριση από `UnBlockUserFromComment`: UnBlockCommentPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.UnBlockUserFromComment`: %v\n", resp)
}
[inline-code-end]

---