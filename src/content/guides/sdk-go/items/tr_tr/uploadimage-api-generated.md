Görüntü yükleme ve yeniden boyutlandırma

## Parametreler

| Ad | Tip | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| sizePreset | string | query | Hayır | Boyut ön ayarı: "Default" (1000x1000px) veya "CrossPlatform" (popüler cihazlar için boyutlar oluşturur) |
| urlId | string | query | Hayır | Yüklemenin yapıldığı sayfanın kimliği, yapılandırma için |

## Yanıt

Döndürür: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_upload_image_response.go)

## Örnek

[inline-code-attrs-start title = 'UploadImage Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sizePreset := openapiclient.SizePreset("Default") // SizePreset | Boyut ön ayarı: \"Default\" (1000x1000px) veya \"CrossPlatform\" (popüler cihazlar için boyutlar oluşturur) (isteğe bağlı)
	urlId := "urlId_example" // string | Yüklemenin yapıldığı sayfanın kimliği, yapılandırma için (isteğe bağlı)

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