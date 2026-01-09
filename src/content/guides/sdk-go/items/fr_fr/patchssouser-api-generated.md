## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| id | string | path | Oui |  |
| updateComments | boolean | query | Non |  |

## Réponse

Renvoie : [`PatchSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_patch_sso_user_api_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple de PatchSSOUser'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	updateComments := true // bool |  (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.PatchSSOUser(context.Background(), id).TenantId(tenantId).UpdateAPISSOUserData(updateAPISSOUserData).UpdateComments(updateComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.PatchSSOUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `PatchSSOUser`: PatchSSOUserAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.PatchSSOUser`: %v\n", resp)
}
[inline-code-end]

---