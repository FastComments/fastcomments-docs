Caricare e ridimensionare un'immagine

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| sizePreset | string | query | No | Preset dimensione: "Default" (1000x1000px) o "CrossPlatform" (crea dimensioni per dispositivi popolari) |
| urlId | string | query | No | ID della pagina da cui avviene il caricamento, per la configurazione |

## Risposta

Restituisce: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_upload_image_response.go)

## Esempio

[inline-code-attrs-start title = 'Esempio di UploadImage'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sizePreset := openapiclient.SizePreset("Default") // SizePreset | Preimpostazione dimensione: \"Default\" (1000x1000px) o \"CrossPlatform\" (crea dimensioni per dispositivi popolari) (opzionale)
	urlId := "urlId_example" // string | ID della pagina da cui avviene il caricamento, per la configurazione (opzionale)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UploadImage(context.Background(), tenantId).File(file).SizePreset(sizePreset).UrlId(urlId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UploadImage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// risposta da `UploadImage`: UploadImageResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UploadImage`: %v\n", resp)
}
[inline-code-end]