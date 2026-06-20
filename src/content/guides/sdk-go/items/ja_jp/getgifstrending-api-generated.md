## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| locale | string | query | No |  |
| rating | string | query | No |  |
| page | number | query | No |  |

## レスポンス

戻り値: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_gifs_trending_response.go)

## 例

[inline-code-attrs-start title = 'GetGifsTrending の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	locale := "locale_example" // string |  (任意)
	rating := "rating_example" // string |  (任意)
	page := float64(1.2) // float64 |  (任意)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetGifsTrending(context.Background(), tenantId).Locale(locale).Rating(rating).Page(page).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetGifsTrending``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetGifsTrending` のレスポンス: GetGifsTrendingResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetGifsTrending`: %v\n", resp)
}
[inline-code-end]