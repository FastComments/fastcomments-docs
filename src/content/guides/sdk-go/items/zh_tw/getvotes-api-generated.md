## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | 查詢 | 是 |  |
| urlId | string | 查詢 | 是 |  |

## 回應

回傳：[`GetVotesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_votes_response.go)

## 範例

[inline-code-attrs-start title = 'GetVotes 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // 字串 | 
	urlId := "urlId_example" // 字串 | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetVotes(context.Background()).TenantId(tenantId).UrlId(urlId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetVotes``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetVotes` 的回應：GetVotesResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetVotes`: %v\n", resp)
}
[inline-code-end]