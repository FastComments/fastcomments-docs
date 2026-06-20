## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_user_search_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-GetSearchUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	value := "value_example" // string |  (אופציונלי)
	sso := "sso_example" // string |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetSearchUsers(context.Background()).Value(value).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetSearchUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`GetSearchUsers`: ModerationUserSearchResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetSearchUsers`: %v\n", resp)
}
[inline-code-end]

---