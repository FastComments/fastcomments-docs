## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| urlId | string | query | Evet |  |

## Yanıt

Döndürür: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_v1_page_likes.go)

## Örnek

[inline-code-attrs-start title = 'GetV1PageLikes Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	resp, r, err := apiClient.PublicAPI.GetV1PageLikes(context.Background(), tenantId).UrlId(urlId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetV1PageLikes``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetV1PageLikes`'den dönen yanıt: GetV1PageLikes
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetV1PageLikes`: %v\n", resp)
}
[inline-code-end]