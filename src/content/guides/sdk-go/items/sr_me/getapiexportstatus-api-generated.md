## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| batchJobId | string | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_export_status_response.go)

## Пример

[inline-code-attrs-start title = 'Пример GetApiExportStatus'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	batchJobId := "batchJobId_example" // string |  (опционо)
	sso := "sso_example" // string |  (опционо)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetApiExportStatus(context.Background()).BatchJobId(batchJobId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetApiExportStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// одговор од `GetApiExportStatus`: ModerationExportStatusResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetApiExportStatus`: %v\n", resp)
}
[inline-code-end]