## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| commentId | string | path | Ja |  |
| broadcastId | string | query | Ja |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_change_comment_pin_status_response.go)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für PinComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.PinComment(context.Background(), tenantId, commentId).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.PinComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `PinComment`: ChangeCommentPinStatusResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.PinComment`: %v\n", resp)
}
[inline-code-end]