## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| userId | string | query | नहीं |  |
| direction | string | query | नहीं |  |
| repliesToUserId | string | query | नहीं |  |
| page | number | query | नहीं |  |
| includei10n | boolean | query | नहीं |  |
| locale | string | query | नहीं |  |
| isCrawler | boolean | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_for_user_response.go)

## उदाहरण

[inline-code-attrs-start title = 'GetCommentsForUser उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	userId := "userId_example" // string |  (वैकल्पिक)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (वैकल्पिक)
	repliesToUserId := "repliesToUserId_example" // string |  (वैकल्पिक)
	page := float64(1.2) // float64 |  (वैकल्पिक)
	includei10n := true // bool |  (वैकल्पिक)
	locale := "locale_example" // string |  (वैकल्पिक)
	isCrawler := true // bool |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsForUser(context.Background()).UserId(userId).Direction(direction).RepliesToUserId(repliesToUserId).Page(page).Includei10n(includei10n).Locale(locale).IsCrawler(isCrawler).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsForUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetCommentsForUser` से प्रतिक्रिया: GetCommentsForUserResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsForUser`: %v\n", resp)
}
[inline-code-end]