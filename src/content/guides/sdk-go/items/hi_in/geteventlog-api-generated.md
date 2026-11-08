req
tenantId
urlId
userIdWS

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| userIdWS | string | query | हाँ |  |
| startTime | integer | query | हाँ |  |
| endTime | integer | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_event_log_response.go)

## उदाहरण

[inline-code-attrs-start title = 'GetEventLog उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userIdWS := "userIdWS_example" // string | 
	startTime := int64(789) // int64 | 
	endTime := int64(789) // int64 |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetEventLog(context.Background(), tenantId).UrlId(urlId).UserIdWS(userIdWS).StartTime(startTime).EndTime(endTime).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetEventLog``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetEventLog` से प्रतिक्रिया: GetEventLogResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetEventLog`: %v\n", resp)
}
[inline-code-end]