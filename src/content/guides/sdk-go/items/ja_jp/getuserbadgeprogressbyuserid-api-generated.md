## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| userId | string | path | はい |  |

## レスポンス

戻り値: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_badge_progress_by_id_200_response.go)

## 例

[inline-code-attrs-start title = 'GetUserBadgeProgressByUserId の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetUserBadgeProgressByUserId(context.Background(), userId).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetUserBadgeProgressByUserId``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetUserBadgeProgressByUserId` からのレスポンス: GetUserBadgeProgressById200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetUserBadgeProgressByUserId`: %v\n", resp)
}
[inline-code-end]

---