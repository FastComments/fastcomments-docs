## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| updateComments | boolean | query | Nej |  |

## Svar

Returnerer: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_put_sso_user_api_response.go)

## Eksempel

[inline-code-attrs-start title = 'PutSSOUser Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	id := "id_example" // string | 
	updateAPISSOUserData := *openapiclient.NewUpdateAPISSOUserData() // UpdateAPISSOUserData | 
	updateComments := true // bool |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.PutSSOUser(context.Background(), id).TenantId(tenantId).UpdateAPISSOUserData(updateAPISSOUserData).UpdateComments(updateComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.PutSSOUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `PutSSOUser`: PutSSOUserAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.PutSSOUser`: %v\n", resp)
}
[inline-code-end]

---