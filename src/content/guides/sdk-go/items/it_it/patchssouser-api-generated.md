## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| id | string | path | Sì |  |
| updateComments | boolean | query | No |  |

## Risposta

Restituisce: [`PatchSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_patch_sso_user_api_response.go)

## Esempio

[inline-code-attrs-start title = 'Esempio PatchSSOUser'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	updateComments := true // bool |  (opzionale)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.PatchSSOUser(context.Background(), id).TenantId(tenantId).UpdateAPISSOUserData(updateAPISSOUserData).UpdateComments(updateComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.PatchSSOUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// risposta da `PatchSSOUser`: PatchSSOUserAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.PatchSSOUser`: %v\n", resp)
}
[inline-code-end]

---