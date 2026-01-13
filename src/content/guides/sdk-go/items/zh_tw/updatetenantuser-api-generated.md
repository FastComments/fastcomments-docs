## 參數

| 名稱 | 類型 | 位置 | 必要 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |
| updateComments | string | query | 否 |  |

## 回應

回傳：[`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_flag_comment_public_200_response.go)

## 範例

[inline-code-attrs-start title = 'UpdateTenantUser 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	id := "id_example" // string | 
	updateTenantUserBody := *openapiclient.NewUpdateTenantUserBody() // UpdateTenantUserBody | 
	updateComments := "updateComments_example" // string |  (可選)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.UpdateTenantUser(context.Background(), id).TenantId(tenantId).UpdateTenantUserBody(updateTenantUserBody).UpdateComments(updateComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.UpdateTenantUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 從 `UpdateTenantUser` 的回應: FlagCommentPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.UpdateTenantUser`: %v\n", resp)
}
[inline-code-end]

---