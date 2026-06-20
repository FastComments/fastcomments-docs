## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| userId | string | query | Nein |  |
| direction | string | query | Nein |  |
| repliesToUserId | string | query | Nein |  |
| page | number | query | Nein |  |
| includei10n | boolean | query | Nein |  |
| locale | string | query | Nein |  |
| isCrawler | boolean | query | Nein |  |

## Antwort

Gibt zurück: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_for_user_response.go)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für GetCommentsForUser'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	userId := "userId_example" // string |  (optional)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (optional)
	repliesToUserId := "repliesToUserId_example" // string |  (optional)
	page := float64(1.2) // float64 |  (optional)
	includei10n := true // bool |  (optional)
	locale := "locale_example" // string |  (optional)
	isCrawler := true // bool |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsForUser(context.Background()).UserId(userId).Direction(direction).RepliesToUserId(repliesToUserId).Page(page).Includei10n(includei10n).Locale(locale).IsCrawler(isCrawler).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsForUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `GetCommentsForUser`: GetCommentsForUserResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsForUser`: %v\n", resp)
}
[inline-code-end]