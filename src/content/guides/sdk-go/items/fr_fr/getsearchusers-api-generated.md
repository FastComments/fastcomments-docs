---
## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| value | string | paramètre de requête | Non |  |
| sso | string | paramètre de requête | Non |  |

## Réponse

Retourne : [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_user_search_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple GetSearchUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	value := "value_example" // string |  (optionnel)
	sso := "sso_example" // string |  (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetSearchUsers(context.Background()).Value(value).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetSearchUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `GetSearchUsers`: ModerationUserSearchResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetSearchUsers`: %v\n", resp)
}
[inline-code-end]

---