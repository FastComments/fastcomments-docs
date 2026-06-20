## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| commentId | string | path | Tak |  |
| broadcastId | string | query | Tak |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_change_comment_pin_status_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład PinComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	commentId := "commentId_example" // string | 
	broadcastId := "broadcastId_example" // string | 
	sso := "sso_example" // string |  (opcjonalne)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.PinComment(context.Background(), tenantId, commentId).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.PinComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `PinComment`: ChangeCommentPinStatusResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.PinComment`: %v\n", resp)
}
[inline-code-end]