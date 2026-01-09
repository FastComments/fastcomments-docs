## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| urlIdWS | string | query | はい |  |
| userIds | string | query | はい |  |

## レスポンス

戻り値: [`GetUserPresenceStatuses200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_presence_statuses_200_response.go)

## 例

[inline-code-attrs-start title = 'GetUserPresenceStatuses の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlIdWS := "urlIdWS_example" // string | 
	userIds := "userIds_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserPresenceStatuses(context.Background()).TenantId(tenantId).UrlIdWS(urlIdWS).UserIds(userIds).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserPresenceStatuses``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetUserPresenceStatuses` のレスポンス: GetUserPresenceStatuses200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserPresenceStatuses`: %v\n", resp)
}
[inline-code-end]