## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| userId | string | query | いいえ |  |
| badgeId | string | query | いいえ |  |
| type | number | query | いいえ |  |
| displayedOnComments | boolean | query | いいえ |  |
| limit | number | query | いいえ |  |
| skip | number | query | いいえ |  |

## レスポンス

戻り値: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_get_user_badges_response.go)

## 例

[inline-code-attrs-start title = 'GetUserBadges の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  （オプション）
	badgeId := "badgeId_example" // string |  （オプション）
	type_ := float64(1.2) // float64 |  （オプション）
	displayedOnComments := true // bool |  （オプション）
	limit := float64(1.2) // float64 |  （オプション）
	skip := float64(1.2) // float64 |  （オプション）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetUserBadges(context.Background()).TenantId(tenantId).UserId(userId).BadgeId(badgeId).Type_(type_).DisplayedOnComments(displayedOnComments).Limit(limit).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetUserBadges``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetUserBadges` のレスポンス: APIGetUserBadgesResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetUserBadges`: %v\n", resp)
}
[inline-code-end]

---