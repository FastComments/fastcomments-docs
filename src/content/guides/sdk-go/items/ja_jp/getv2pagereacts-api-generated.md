## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | パス | はい |  |
| urlId | string | クエリ | はい |  |

## レスポンス

返却値: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_v2_page_reacts.go)

## 例

[inline-code-attrs-start title = 'GetV2PageReacts の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetV2PageReacts(context.Background(), tenantId).UrlId(urlId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetV2PageReacts``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetV2PageReacts` のレスポンス: GetV2PageReacts
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetV2PageReacts`: %v\n", resp)
}
[inline-code-end]