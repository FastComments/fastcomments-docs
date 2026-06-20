## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_empty_response.go)

## Örnek

[inline-code-attrs-start title = 'PostBanUserUndo Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	banUserUndoParams := *openapiclient.NewBanUserUndoParams(*openapiclient.NewAPIBanUserChangeLog()) // BanUserUndoParams | 
	sso := "sso_example" // string |  (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostBanUserUndo(context.Background()).BanUserUndoParams(banUserUndoParams).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostBanUserUndo``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `PostBanUserUndo`'den gelen yanıt: APIEmptyResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostBanUserUndo`: %v\n", resp)
}
[inline-code-end]