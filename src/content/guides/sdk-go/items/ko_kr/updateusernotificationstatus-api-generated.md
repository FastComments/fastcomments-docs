## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| notificationId | string | path | 예 |  |
| newStatus | string | path | 예 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_update_user_notification_status_200_response.go)

## 예제

[inline-code-attrs-start title = 'UpdateUserNotificationStatus 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	notificationId := "notificationId_example" // string | 
	newStatus := "newStatus_example" // string | 
	sso := "sso_example" // string |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UpdateUserNotificationStatus(context.Background(), notificationId, newStatus).TenantId(tenantId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UpdateUserNotificationStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `UpdateUserNotificationStatus`의 응답: UpdateUserNotificationStatus200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UpdateUserNotificationStatus`: %v\n", resp)
}
[inline-code-end]