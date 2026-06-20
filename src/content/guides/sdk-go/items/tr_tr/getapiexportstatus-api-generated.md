---
## Parametreler

| İsim | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| batchJobId | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_export_status_response.go)

## Örnek

[inline-code-attrs-start title = 'GetApiExportStatus Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	batchJobId := "batchJobId_example" // string |  (isteğe bağlı)
	sso := "sso_example" // string |  (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetApiExportStatus(context.Background()).BatchJobId(batchJobId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetApiExportStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetApiExportStatus`'ten dönen yanıt: ModerationExportStatusResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetApiExportStatus`: %v\n", resp)
}
[inline-code-end]

---