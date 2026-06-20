## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| urlId | string | query | Да |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_empty_response.go)

## Пример

[inline-code-attrs-start title = 'Пример за PutReopenThread'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	urlId := "urlId_example" // string | 
	sso := "sso_example" // string |  (по избор)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PutReopenThread(context.Background()).UrlId(urlId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PutReopenThread``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// отговор от `PutReopenThread`: APIEmptyResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PutReopenThread`: %v\n", resp)
}
[inline-code-end]