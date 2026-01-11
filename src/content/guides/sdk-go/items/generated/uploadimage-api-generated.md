Upload and resize an image

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Size preset: "Default" (1000x1000px) or "CrossPlatform" (creates sizes for popular devices) |
| urlId | string | query | No | Page id that upload is happening from, to configure |

## Response

Returns: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_upload_image_response.go)

## Example

[inline-code-attrs-start title = 'UploadImage Example'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	file := os.NewFile(1234, "some_file") // *os.File | 
	sizePreset := openapiclient.SizePreset("Default") // SizePreset | Size preset: \"Default\" (1000x1000px) or \"CrossPlatform\" (creates sizes for popular devices) (optional)
	urlId := "urlId_example" // string | Page id that upload is happening from, to configure (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UploadImage(context.Background(), tenantId).File(file).SizePreset(sizePreset).UrlId(urlId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UploadImage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UploadImage`: UploadImageResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UploadImage`: %v\n", resp)
}
[inline-code-end]
