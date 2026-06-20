## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| largeInternalURLSanitized | string | query | 是 |  |

## 回應

回傳: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_gif_get_large_response.go)

## 範例

[inline-code-attrs-start title = 'GetGifLarge 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	largeInternalURLSanitized := "largeInternalURLSanitized_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetGifLarge(context.Background(), tenantId).LargeInternalURLSanitized(largeInternalURLSanitized).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetGifLarge``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 回應自 `GetGifLarge`: GifGetLargeResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetGifLarge`: %v\n", resp)
}
[inline-code-end]