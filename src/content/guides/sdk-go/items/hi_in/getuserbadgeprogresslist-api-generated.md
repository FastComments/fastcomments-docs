## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| userId | string | query | नहीं |  |
| limit | number | query | नहीं |  |
| skip | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_get_user_badge_progress_list_response.go)

## उदाहरण

[inline-code-attrs-start title = 'GetUserBadgeProgressList उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	limit := float64(1.2) // float64 |  (वैकल्पिक)
	skip := float64(1.2) // float64 |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetUserBadgeProgressList(context.Background()).TenantId(tenantId).UserId(userId).Limit(limit).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetUserBadgeProgressList``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetUserBadgeProgressList` से प्रतिक्रिया: APIGetUserBadgeProgressListResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetUserBadgeProgressList`: %v\n", resp)
}
[inline-code-end]