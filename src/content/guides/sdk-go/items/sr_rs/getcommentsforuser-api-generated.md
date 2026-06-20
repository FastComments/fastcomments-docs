## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Не |  |
| direction | string | query | Не |  |
| repliesToUserId | string | query | Не |  |
| page | number | query | Не |  |
| includei10n | boolean | query | Не |  |
| locale | string | query | Не |  |
| isCrawler | boolean | query | Не |  |

## Одговор

Враћа: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_for_user_response.go)

## Пример

[inline-code-attrs-start title = 'Пример за GetCommentsForUser'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	userId := "userId_example" // string |  (опционо)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (опционо)
	repliesToUserId := "repliesToUserId_example" // string |  (опционо)
	page := float64(1.2) // float64 |  (опционо)
	includei10n := true // bool |  (опционо)
	locale := "locale_example" // string |  (опционо)
	isCrawler := true // bool |  (опционо)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsForUser(context.Background()).UserId(userId).Direction(direction).RepliesToUserId(repliesToUserId).Page(page).Includei10n(includei10n).Locale(locale).IsCrawler(isCrawler).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsForUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// одговор од `GetCommentsForUser`: GetCommentsForUserResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsForUser`: %v\n", resp)
}
[inline-code-end]

---