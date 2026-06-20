## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | putanja | Da |  |
| commentId | string | putanja | Da |  |
| broadcastId | string | upit | Da |  |
| sso | string | upit | Ne |  |

## Odgovor

Vraća: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_change_comment_pin_status_response.go)

## Primjer

[inline-code-attrs-start title = 'Primjer UnPinComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (neobvezno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UnPinComment(context.Background(), tenantId, commentId).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UnPinComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor iz `UnPinComment`: ChangeCommentPinStatusResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UnPinComment`: %v\n", resp)
}
[inline-code-end]

---