## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| usernameStartsWith | string | query | नहीं |  |
| mentionGroupIds | array | query | नहीं |  |
| sso | string | query | नहीं |  |
| searchSection | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_search_users_result.go)

## उदाहरण

[inline-code-attrs-start title = 'SearchUsers उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 
	usernameStartsWith := "usernameStartsWith_example" // string |  (वैकल्पिक)
	mentionGroupIds := []string{"Inner_example"} // []string |  (वैकल्पिक)
	sso := "sso_example" // string |  (वैकल्पिक)
	searchSection := "searchSection_example" // string |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.SearchUsers(context.Background(), tenantId).UrlId(urlId).UsernameStartsWith(usernameStartsWith).MentionGroupIds(mentionGroupIds).Sso(sso).SearchSection(searchSection).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.SearchUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `SearchUsers` से प्रतिक्रिया: SearchUsersResult
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.SearchUsers`: %v\n", resp)
}
[inline-code-end]