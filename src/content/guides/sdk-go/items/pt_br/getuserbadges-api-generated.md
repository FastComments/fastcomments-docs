## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| userId | string | query | Não |  |
| badgeId | string | query | Não |  |
| type | number | query | Não |  |
| displayedOnComments | boolean | query | Não |  |
| limit | number | query | Não |  |
| skip | number | query | Não |  |

## Resposta

Retorna: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_badges_200_response.go)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de GetUserBadges'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (opcional)
	badgeId := "badgeId_example" // string |  (opcional)
	type_ := float64(1.2) // float64 |  (opcional)
	displayedOnComments := true // bool |  (opcional)
	limit := float64(1.2) // float64 |  (opcional)
	skip := float64(1.2) // float64 |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetUserBadges(context.Background()).TenantId(tenantId).UserId(userId).BadgeId(badgeId).Type_(type_).DisplayedOnComments(displayedOnComments).Limit(limit).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetUserBadges``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// resposta de `GetUserBadges`: GetUserBadges200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetUserBadges`: %v\n", resp)
}
[inline-code-end]

---