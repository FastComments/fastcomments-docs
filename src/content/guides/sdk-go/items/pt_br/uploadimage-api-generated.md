Enviar e redimensionar uma imagem

## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Predefinição de tamanho: "Default" (1000x1000px) ou "CrossPlatform" (cria tamanhos para dispositivos populares) |
| urlId | string | query | No | ID da página de onde o upload está sendo feito, para configurar |

## Resposta

Retorna: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_upload_image_response.go)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de UploadImage'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sizePreset := openapiclient.SizePreset("Default") // SizePreset | Predefinição de tamanho: \"Default\" (1000x1000px) ou \"CrossPlatform\" (cria tamanhos para dispositivos populares) (opcional)
	urlId := "urlId_example" // string | ID da página de onde o upload está sendo feito, para configurar (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UploadImage(context.Background(), tenantId).File(file).SizePreset(sizePreset).UrlId(urlId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UploadImage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// resposta de `UploadImage`: UploadImageResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UploadImage`: %v\n", resp)
}
[inline-code-end]