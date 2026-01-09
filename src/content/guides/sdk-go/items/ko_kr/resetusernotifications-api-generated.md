## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| afterId | string | query | 아니요 |  |
| afterCreatedAt | integer | query | 아니요 |  |
| unreadOnly | boolean | query | 아니요 |  |
| dmOnly | boolean | query | 아니요 |  |
| noDm | boolean | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_reset_user_notifications_200_response.go)

## 예제

[inline-code-attrs-start title = 'ResetUserNotifications 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterId := "afterId_example" // string |  (선택 사항)
	afterCreatedAt := int64(789) // int64 |  (선택 사항)
	unreadOnly := true // bool |  (선택 사항)
	dmOnly := true // bool |  (선택 사항)
	noDm := true // bool |  (선택 사항)
	sso := "sso_example" // string |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.ResetUserNotifications(context.Background()).TenantId(tenantId).AfterId(afterId).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.ResetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `ResetUserNotifications`의 응답: ResetUserNotifications200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.ResetUserNotifications`: %v\n", resp)
}
[inline-code-end]