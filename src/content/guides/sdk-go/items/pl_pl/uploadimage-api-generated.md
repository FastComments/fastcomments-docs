Prześlij i zmień rozmiar obrazu

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| sizePreset | string | query | Nie | Ustawienie rozmiaru: "Default" (1000x1000px) lub "CrossPlatform" (tworzy rozmiary dla popularnych urządzeń) |
| urlId | string | query | Nie | ID strony, z której odbywa się przesyłanie, do konfiguracji |

## Odpowiedź

Zwraca: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_upload_image_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład UploadImage'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sizePreset := openapiclient.SizePreset("Default") // SizePreset | Ustawienie rozmiaru: \"Default\" (1000x1000px) lub \"CrossPlatform\" (tworzy rozmiary dla popularnych urządzeń) (opcjonalne)
	urlId := "urlId_example" // string | ID strony, z której odbywa się przesyłanie, do konfiguracji (opcjonalne)

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

---