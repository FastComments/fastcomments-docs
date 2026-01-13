---
Upload and resize an image

## Parameters

| Naam | Type | Location | Verplicht | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| sizePreset | string | query | Nee | Groottevoorinstelling: "Default" (1000x1000px) of "CrossPlatform" (maakt formaten voor populaire apparaten) |
| urlId | string | query | Nee | Pagina-id waarvan de upload plaatsvindt, voor configuratie |

## Response

Retourneert: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_upload_image_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'UploadImage Voorbeeld'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sizePreset := openapiclient.SizePreset("Default") // SizePreset | Groottevoorinstelling: \"Default\" (1000x1000px) of \"CrossPlatform\" (maakt formaten voor populaire apparaten) (optioneel)
	urlId := "urlId_example" // string | Pagina-id waarvan de upload plaatsvindt, voor configuratie (optioneel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UploadImage(context.Background(), tenantId).File(file).SizePreset(sizePreset).UrlId(urlId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UploadImage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// respons van `UploadImage`: UploadImageResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UploadImage`: %v\n", resp)
}
[inline-code-end]

---