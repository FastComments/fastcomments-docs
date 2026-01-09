## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |
| userId | string | query | いいえ |  |

## レスポンス

戻り値: [`DeleteSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_delete_subscription_api_response.go)

## 例

[inline-code-attrs-start title = 'DeleteSubscription の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (任意)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.DeleteSubscription(context.Background(), id).TenantId(tenantId).UserId(userId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.DeleteSubscription``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `DeleteSubscription` からのレスポンス: DeleteSubscriptionAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.DeleteSubscription`: %v\n", resp)
}
[inline-code-end]