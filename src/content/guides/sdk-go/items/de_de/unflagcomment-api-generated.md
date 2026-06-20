## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| userId | string | query | Nein |  |
| anonUserId | string | query | Nein |  |

## Antwort

Gibt zurück: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_flag_comment_response.go)

## Beispiel

[inline-code-attrs-start title = 'UnFlagComment Beispiel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	id := "id_example" // string | 
	userId := "userId_example" // string |  (optional)
	anonUserId := "anonUserId_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.UnFlagComment(context.Background(), id).TenantId(tenantId).UserId(userId).AnonUserId(anonUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.UnFlagComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `UnFlagComment`: FlagCommentResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.UnFlagComment`: %v\n", resp)
}
[inline-code-end]