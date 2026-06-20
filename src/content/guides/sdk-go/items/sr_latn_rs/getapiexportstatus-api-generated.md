---
## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| batchJobId | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_export_status_response.go)

## Primer

[inline-code-attrs-start title = 'Primer GetApiExportStatus'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	batchJobId := "batchJobId_example" // string |  (neobavezno)
	sso := "sso_example" // string |  (neobavezno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetApiExportStatus(context.Background()).BatchJobId(batchJobId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetApiExportStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor od `GetApiExportStatus`: ModerationExportStatusResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetApiExportStatus`: %v\n", resp)
}
[inline-code-end]

---