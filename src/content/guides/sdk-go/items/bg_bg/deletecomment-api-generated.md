## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| contextUserId | string | query | Не |  |
| isLive | boolean | query | Не |  |

## Отговор

Връща: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_delete_comment_result.go)

## Пример

[inline-code-attrs-start title = 'Пример за DeleteComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	contextUserId := "contextUserId_example" // string |  (незадължително)
	isLive := true // bool |  (незадължително)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.DeleteComment(context.Background(), id).TenantId(tenantId).ContextUserId(contextUserId).IsLive(isLive).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.DeleteComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DeleteComment`: DeleteCommentResult
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.DeleteComment`: %v\n", resp)
}
[inline-code-end]