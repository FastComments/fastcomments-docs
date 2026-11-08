## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| userId | string | query | नहीं |  |
| badgeId | string | query | नहीं |  |
| type | number | query | नहीं |  |
| displayedOnComments | boolean | query | नहीं |  |
| limit | number | query | नहीं |  |
| skip | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_get_user_badges_response.go)

## उदाहरण

[inline-code-attrs-start title = 'GetUserBadges उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	badgeId := "badgeId_example" // string |  (वैकल्पिक)
	type_ := float64(1.2) // float64 |  (वैकल्पिक)
	displayedOnComments := true // bool |  (वैकल्पिक)
	limit := float64(1.2) // float64 |  (वैकल्पिक)
	skip := float64(1.2) // float64 |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetUserBadges(context.Background()).TenantId(tenantId).UserId(userId).BadgeId(badgeId).Type_(type_).DisplayedOnComments(displayedOnComments).Limit(limit).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetUserBadges``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetUserBadges` से प्रतिक्रिया: APIGetUserBadgesResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetUserBadges`: %v\n", resp)
}
[inline-code-end]