## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| afterId | string | query | नहीं |  |
| afterCreatedAt | integer | query | नहीं |  |
| unreadOnly | boolean | query | नहीं |  |
| dmOnly | boolean | query | नहीं |  |
| noDm | boolean | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_reset_user_notifications_response.go)

## उदाहरण

[inline-code-attrs-start title = 'ResetUserNotifications उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterId := "afterId_example" // string |  (वैकल्पिक)
	afterCreatedAt := int64(789) // int64 |  (वैकल्पिक)
	unreadOnly := true // bool |  (वैकल्पिक)
	dmOnly := true // bool |  (वैकल्पिक)
	noDm := true // bool |  (वैकल्पिक)
	sso := "sso_example" // string |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.ResetUserNotifications(context.Background()).TenantId(tenantId).AfterId(afterId).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.ResetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `ResetUserNotifications` से प्रतिक्रिया: ResetUserNotificationsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.ResetUserNotifications`: %v\n", resp)
}
[inline-code-end]