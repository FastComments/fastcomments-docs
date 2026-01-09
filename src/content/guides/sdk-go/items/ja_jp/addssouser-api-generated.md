## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |

## レスポンス

戻り値: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_add_sso_user_api_response.go)

## 例

[inline-code-attrs-start title = 'AddSSOUser の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createAPISSOUserData := *openapiclient.NewCreateAPISSOUserData("Email_example", "Username_example", "Id_example") // CreateAPISSOUserData | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.AddSSOUser(context.Background()).TenantId(tenantId).CreateAPISSOUserData(createAPISSOUserData).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.AddSSOUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `AddSSOUser` のレスポンス: AddSSOUserAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.AddSSOUser`: %v\n", resp)
}
[inline-code-end]

---