## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| urlId | string | query | כן |  |
| usernameStartsWith | string | query | לא |  |
| mentionGroupIds | array | query | לא |  |
| sso | string | query | לא |  |
| searchSection | string | query | לא |  |

## תגובה

מחזיר: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_search_users_200_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-SearchUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // מחרוזת | 
	urlId := "urlId_example" // מחרוזת | 
	usernameStartsWith := "usernameStartsWith_example" // מחרוזת |  (אופציונלי)
	mentionGroupIds := []string{"Inner_example"} // []מחרוזת |  (אופציונלי)
	sso := "sso_example" // מחרוזת |  (אופציונלי)
	searchSection := "searchSection_example" // מחרוזת |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.SearchUsers(context.Background(), tenantId).UrlId(urlId).UsernameStartsWith(usernameStartsWith).MentionGroupIds(mentionGroupIds).Sso(sso).SearchSection(searchSection).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.SearchUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`SearchUsers`: SearchUsers200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.SearchUsers`: %v\n", resp)
}
[inline-code-end]