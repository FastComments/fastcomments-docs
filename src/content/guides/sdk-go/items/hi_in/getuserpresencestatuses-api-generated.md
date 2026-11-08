## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| urlIdWS | string | query | हाँ |  |
| userIds | string | query | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_presence_statuses_response.go)

## उदाहरण

[inline-code-attrs-start title = 'GetUserPresenceStatuses उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlIdWS := "urlIdWS_example" // string | 
	userIds := "userIds_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserPresenceStatuses(context.Background()).TenantId(tenantId).UrlIdWS(urlIdWS).UserIds(userIds).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserPresenceStatuses``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetUserPresenceStatuses` से प्रतिक्रिया: GetUserPresenceStatusesResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserPresenceStatuses`: %v\n", resp)
}
[inline-code-end]