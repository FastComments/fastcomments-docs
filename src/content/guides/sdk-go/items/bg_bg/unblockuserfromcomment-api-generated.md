## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| userId | string | query | Не |  |
| anonUserId | string | query | Не |  |

## Отговор

Връща: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_unblock_success.go)

## Пример

[inline-code-attrs-start title = 'Пример за UnBlockUserFromComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (незадължително)
	anonUserId := "anonUserId_example" // string |  (незадължително)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.UnBlockUserFromComment(context.Background(), id).TenantId(tenantId).UnBlockFromCommentParams(unBlockFromCommentParams).UserId(userId).AnonUserId(anonUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.UnBlockUserFromComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UnBlockUserFromComment`: UnblockSuccess
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.UnBlockUserFromComment`: %v\n", resp)
}
[inline-code-end]