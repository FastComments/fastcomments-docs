## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| id | string | path | 예 |  |
| contextUserId | string | query | 아니요 |  |
| isLive | boolean | query | 아니요 |  |

## 응답

반환: [`DeleteComment200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_delete_comment_200_response.go)

## 예제

[inline-code-attrs-start title = 'DeleteComment 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	contextUserId := "contextUserId_example" // string |  (선택 사항)
	isLive := true // bool |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.DeleteComment(context.Background(), id).TenantId(tenantId).ContextUserId(contextUserId).IsLive(isLive).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.DeleteComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `DeleteComment`의 응답: DeleteComment200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.DeleteComment`: %v\n", resp)
}
[inline-code-end]

---