## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| urlId | string | query | 아니오 | 페이지가 구독되어 있는지 여부를 결정하는 데 사용합니다. |
| pageSize | integer | query | 아니오 |  |
| afterId | string | query | 아니오 |  |
| includeContext | boolean | query | 아니오 |  |
| afterCreatedAt | integer | query | 아니오 |  |
| unreadOnly | boolean | query | 아니오 |  |
| dmOnly | boolean | query | 아니오 |  |
| noDm | boolean | query | 아니오 |  |
| includeTranslations | boolean | query | 아니오 |  |
| includeTenantNotifications | boolean | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_my_notifications_response.go)

## 예제

[inline-code-attrs-start title = 'GetUserNotifications 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 현재 페이지가 구독되어 있는지 여부를 결정하는 데 사용합니다. (선택 사항)
	pageSize := int32(56) // int32 |  (선택 사항)
	afterId := "afterId_example" // string |  (선택 사항)
	includeContext := true // bool |  (선택 사항)
	afterCreatedAt := int64(789) // int64 |  (선택 사항)
	unreadOnly := true // bool |  (선택 사항)
	dmOnly := true // bool |  (선택 사항)
	noDm := true // bool |  (선택 사항)
	includeTranslations := true // bool |  (선택 사항)
	includeTenantNotifications := true // bool |  (선택 사항)
	sso := "sso_example" // string |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserNotifications(context.Background()).TenantId(tenantId).UrlId(urlId).PageSize(pageSize).AfterId(afterId).IncludeContext(includeContext).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).IncludeTranslations(includeTranslations).IncludeTenantNotifications(includeTenantNotifications).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetUserNotifications`의 응답: GetMyNotificationsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserNotifications`: %v\n", resp)
}
[inline-code-end]