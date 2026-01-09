## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| id | string | path | 예 |  |
| deleteComments | boolean | query | 아니요 |  |
| commentDeleteMode | string | query | 아니요 |  |

## 응답

반환: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_delete_sso_user_api_response.go)

## 예제

[inline-code-attrs-start title = 'DeleteSSOUser 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	deleteComments := true // bool |  (선택 사항)
	commentDeleteMode := "commentDeleteMode_example" // string |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.DeleteSSOUser(context.Background(), id).TenantId(tenantId).DeleteComments(deleteComments).CommentDeleteMode(commentDeleteMode).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.DeleteSSOUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `DeleteSSOUser`의 응답: DeleteSSOUserAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.DeleteSSOUser`: %v\n", resp)
}
[inline-code-end]