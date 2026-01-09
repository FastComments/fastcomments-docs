## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| postIds | array | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`GetUserReactsPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_reacts_public_200_response.go)

## Eksempel

[inline-code-attrs-start title = 'GetUserReactsPublic Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	postIds := []string{"Inner_example"} // []string |  (valgfri)
	sso := "sso_example" // string |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserReactsPublic(context.Background(), tenantId).PostIds(postIds).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserReactsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `GetUserReactsPublic`: GetUserReactsPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserReactsPublic`: %v\n", resp)
}
[inline-code-end]

---