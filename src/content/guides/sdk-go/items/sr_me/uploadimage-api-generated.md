Otpremi i promijeni veličinu slike

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Predložak veličine: "Default" (1000x1000px) ili "CrossPlatform" (stvara veličine za popularne uređaje) |
| urlId | string | query | No | ID stranice sa koje se vrši otpremanje, za konfiguraciju |

## Odgovor

Vraća: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_upload_image_response.go)

## Primjer

[inline-code-attrs-start title = 'Primjer UploadImage-a'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	file := os.NewFile(1234, "some_file") // *os.File | 
	sizePreset := openapiclient.SizePreset("Default") // SizePreset | Predložak veličine: \"Default\" (1000x1000px) ili \"CrossPlatform\" (stvara veličine za popularne uređaje) (opciono)
	urlId := "urlId_example" // string | ID stranice sa koje se vrši otpremanje, za konfiguraciju (opciono)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UploadImage(context.Background(), tenantId).File(file).SizePreset(sizePreset).UrlId(urlId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UploadImage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor od `UploadImage`: UploadImageResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UploadImage`: %v\n", resp)
}
[inline-code-end]