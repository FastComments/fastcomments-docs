## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Non |  |
| direction | string | query | Non |  |
| repliesToUserId | string | query | Non |  |
| page | number | query | Non |  |
| includei10n | boolean | query | Non |  |
| locale | string | query | Non |  |
| isCrawler | boolean | query | Non |  |

## Réponse

Renvoie : [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_for_user_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple GetCommentsForUser'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	userId := "userId_example" // string |  (optionnel)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (optionnel)
	repliesToUserId := "repliesToUserId_example" // string |  (optionnel)
	page := float64(1.2) // float64 |  (optionnel)
	includei10n := true // bool |  (optionnel)
	locale := "locale_example" // string |  (optionnel)
	isCrawler := true // bool |  (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsForUser(context.Background()).UserId(userId).Direction(direction).RepliesToUserId(repliesToUserId).Page(page).Includei10n(includei10n).Locale(locale).IsCrawler(isCrawler).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsForUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `GetCommentsForUser` : GetCommentsForUserResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsForUser`: %v\n", resp)
}
[inline-code-end]