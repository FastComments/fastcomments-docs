## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Nej |  |
| direction | string | query | Nej |  |
| repliesToUserId | string | query | Nej |  |
| page | number | query | Nej |  |
| includei10n | boolean | query | Nej |  |
| locale | string | query | Nej |  |
| isCrawler | boolean | query | Nej |  |

## Svar

Returnerer: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_for_user_response.go)

## Eksempel

[inline-code-attrs-start title = 'GetCommentsForUser Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	userId := "userId_example" // string |  (valgfri)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (valgfri)
	repliesToUserId := "repliesToUserId_example" // string |  (valgfri)
	page := float64(1.2) // float64 |  (valgfri)
	includei10n := true // bool |  (valgfri)
	locale := "locale_example" // string |  (valgfri)
	isCrawler := true // bool |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsForUser(context.Background()).UserId(userId).Direction(direction).RepliesToUserId(repliesToUserId).Page(page).Includei10n(includei10n).Locale(locale).IsCrawler(isCrawler).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsForUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `GetCommentsForUser`: GetCommentsForUserResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsForUser`: %v\n", resp)
}
[inline-code-end]