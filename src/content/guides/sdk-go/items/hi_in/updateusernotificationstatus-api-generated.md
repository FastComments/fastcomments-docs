## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| notificationId | string | path | हाँ |  |
| newStatus | string | path | हाँ |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_update_user_notification_status_response.go)

## उदाहरण

[inline-code-attrs-start title = 'UpdateUserNotificationStatus उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	notificationId := "notificationId_example" // string | 
	newStatus := "newStatus_example" // string | 
	sso := "sso_example" // string |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UpdateUserNotificationStatus(context.Background(), notificationId, newStatus).TenantId(tenantId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UpdateUserNotificationStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UpdateUserNotificationStatus`: UpdateUserNotificationStatusResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UpdateUserNotificationStatus`: %v\n", resp)
}
[inline-code-end]