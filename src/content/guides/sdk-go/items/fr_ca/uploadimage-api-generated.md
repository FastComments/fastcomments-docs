---
Téléverser et redimensionner une image

## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Préréglage de taille : "Default" (1000x1000px) ou "CrossPlatform" (crée des tailles pour les appareils populaires) |
| urlId | string | query | No | ID de la page à partir de laquelle le téléversement est effectué, pour la configuration |

## Réponse

Renvoie : [`UploadImageResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_upload_image_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple UploadImage'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sizePreset := openapiclient.SizePreset("Default") // SizePreset | Préréglage de taille : \"Default\" (1000x1000px) ou \"CrossPlatform\" (crée des tailles pour les appareils populaires) (optionnel)
	urlId := "urlId_example" // string | ID de la page à partir de laquelle le téléversement est effectué, pour la configuration (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UploadImage(context.Background(), tenantId).File(file).SizePreset(sizePreset).UrlId(urlId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UploadImage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `UploadImage`: UploadImageResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UploadImage`: %v\n", resp)
}
[inline-code-end]

---