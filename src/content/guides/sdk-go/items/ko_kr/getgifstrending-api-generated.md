## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| locale | string | query | 아니오 |  |
| rating | string | query | 아니오 |  |
| page | number | query | 아니오 |  |

## 응답

반환: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_gifs_trending_response.go)

## 예제

[inline-code-attrs-start title = 'GetGifsTrending 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	locale := "locale_example" // string |  (선택 사항)
	rating := "rating_example" // string |  (선택 사항)
	page := float64(1.2) // float64 |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetGifsTrending(context.Background(), tenantId).Locale(locale).Rating(rating).Page(page).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetGifsTrending``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetGifsTrending`의 응답: GetGifsTrendingResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetGifsTrending`: %v\n", resp)
}
[inline-code-end]