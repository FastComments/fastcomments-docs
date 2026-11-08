## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| userId | string | query | नहीं |  |
| urlId | string | query | नहीं |  |
| fromCommentId | string | query | नहीं |  |
| viewed | boolean | query | नहीं |  |
| type | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_notification_count_response.go)

## उदाहरण

[inline-code-attrs-start title = 'GetNotificationCount उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (वैकल्पिक)
	urlId := "urlId_example" // string |  (वैकल्पिक)
	fromCommentId := "fromCommentId_example" // string |  (वैकल्पिक)
	viewed := true // bool |  (वैकल्पिक)
	type_ := "type__example" // string |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetNotificationCount(context.Background()).TenantId(tenantId).UserId(userId).UrlId(urlId).FromCommentId(fromCommentId).Viewed(viewed).Type_(type_).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetNotificationCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetNotificationCount`: GetNotificationCountResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetNotificationCount`: %v\n", resp)
}
[inline-code-end]