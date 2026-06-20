## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| sso | string | query | Nej |  |

## Respons

Returnerer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_empty_response.go)

## Eksempel

[inline-code-attrs-start title = 'PostUnFlagComment eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	commentId := "commentId_example" // string | 
	sso := "sso_example" // string |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostUnFlagComment(context.Background(), commentId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostUnFlagComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `PostUnFlagComment`: APIEmptyResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostUnFlagComment`: %v\n", resp)
}
[inline-code-end]