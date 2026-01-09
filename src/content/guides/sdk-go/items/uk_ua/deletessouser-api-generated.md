## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |
| deleteComments | boolean | query | Ні |  |
| commentDeleteMode | string | query | Ні |  |

## Відповідь

Повертає: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_delete_sso_user_api_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад DeleteSSOUser'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	deleteComments := true // bool |  (необов'язково)
	commentDeleteMode := "commentDeleteMode_example" // string |  (необов'язково)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.DeleteSSOUser(context.Background(), id).TenantId(tenantId).DeleteComments(deleteComments).CommentDeleteMode(commentDeleteMode).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.DeleteSSOUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь від `DeleteSSOUser`: DeleteSSOUserAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.DeleteSSOUser`: %v\n", resp)
}
[inline-code-end]