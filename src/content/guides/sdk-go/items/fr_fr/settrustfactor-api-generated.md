## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| trustFactor | string | query | No |  |
| sso | string | query | No |  |

## Réponse

Renvoie : [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_set_user_trust_factor_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple SetTrustFactor'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (optionnel)
	trustFactor := "trustFactor_example" // string |  (optionnel)
	sso := "sso_example" // string |  (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.SetTrustFactor(context.Background()).TenantId(tenantId).UserId(userId).TrustFactor(trustFactor).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Erreur lors de l'appel de `ModerationAPI.SetTrustFactor`` : %v\n", err)
		fmt.Fprintf(os.Stderr, "Réponse HTTP complète : %v\n", r)
	}
	// réponse de `SetTrustFactor` : SetUserTrustFactorResponse
	fmt.Fprintf(os.Stdout, "Réponse de `ModerationAPI.SetTrustFactor` : %v\n", resp)
}
[inline-code-end]