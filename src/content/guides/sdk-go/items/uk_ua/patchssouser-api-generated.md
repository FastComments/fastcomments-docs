## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |
| updateComments | boolean | query | Ні |  |

## Відповідь

Повертає: [`PatchSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_patch_sso_user_api_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад PatchSSOUser'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	updateAPISSOUserData := *openapiclient.NewUpdateAPISSOUserData() // UpdateAPISSOUserData | 
	updateComments := true // bool |  (необов'язково)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.PatchSSOUser(context.Background(), id).TenantId(tenantId).UpdateAPISSOUserData(updateAPISSOUserData).UpdateComments(updateComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.PatchSSOUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь від `PatchSSOUser`: PatchSSOUserAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.PatchSSOUser`: %v\n", resp)
}
[inline-code-end]