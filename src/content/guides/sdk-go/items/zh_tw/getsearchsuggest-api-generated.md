## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| text-search | string | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_suggest_response.go)

## 範例

[inline-code-attrs-start title = 'GetSearchSuggest 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	textSearch := "textSearch_example" // string |  （可選）
	sso := "sso_example" // string |  （可選）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetSearchSuggest(context.Background()).TextSearch(textSearch).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetSearchSuggest``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 從 `GetSearchSuggest` 的回應：ModerationSuggestResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetSearchSuggest`: %v\n", resp)
}
[inline-code-end]

---