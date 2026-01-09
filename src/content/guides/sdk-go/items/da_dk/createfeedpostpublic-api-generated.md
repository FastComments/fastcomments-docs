## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| broadcastId | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`CreateFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_feed_post_public_200_response.go)

## Eksempel

[inline-code-attrs-start title = 'CreateFeedPostPublic Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	createFeedPostParams := *openapiclient.NewCreateFeedPostParams() // CreateFeedPostParams | 
	broadcastId := "broadcastId_example" // string |  (valgfri)
	sso := "sso_example" // string |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.CreateFeedPostPublic(context.Background(), tenantId).CreateFeedPostParams(createFeedPostParams).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.CreateFeedPostPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `CreateFeedPostPublic`: CreateFeedPostPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.CreateFeedPostPublic`: %v\n", resp)
}
[inline-code-end]