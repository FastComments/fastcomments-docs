## 參數

| 名稱 | Type | Location | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |

## 回應

回傳: [`DeletePageAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_delete_page_api_response.go)

## 範例

[inline-code-attrs-start title = 'DeletePage 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.DeletePage(context.Background(), id).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.DeletePage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 來自 `DeletePage` 的回應：DeletePageAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.DeletePage`: %v\n", resp)
}
[inline-code-end]

---