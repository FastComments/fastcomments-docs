## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| largeInternalURLSanitized | string | query | כן |  |

## תגובה

מחזיר: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_gif_get_large_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-GetGifLarge'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	// תשובה מ-`GetGifLarge`: GifGetLargeResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetGifLarge`: %v\n", resp)
}
[inline-code-end]