## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| userId | string | query | Όχι |  |
| direction | string | query | Όχι |  |
| repliesToUserId | string | query | Όχι |  |
| page | number | query | Όχι |  |
| includei10n | boolean | query | Όχι |  |
| locale | string | query | Όχι |  |
| isCrawler | boolean | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_for_user_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα GetCommentsForUser'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	userId := "userId_example" // string |  (προαιρετικό)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (προαιρετικό)
	repliesToUserId := "repliesToUserId_example" // string |  (προαιρετικό)
	page := float64(1.2) // float64 |  (προαιρετικό)
	includei10n := true // bool |  (προαιρετικό)
	locale := "locale_example" // string |  (προαιρετικό)
	isCrawler := true // bool |  (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsForUser(context.Background()).UserId(userId).Direction(direction).RepliesToUserId(repliesToUserId).Page(page).Includei10n(includei10n).Locale(locale).IsCrawler(isCrawler).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsForUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απόκριση από `GetCommentsForUser`: GetCommentsForUserResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsForUser`: %v\n", resp)
}
[inline-code-end]

---