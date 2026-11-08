## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| limit | number | query | नहीं |  |
| skip | number | query | नहीं |  |
| order | string | query | नहीं |  |
| after | number | query | नहीं |  |
| before | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_audit_logs_response.go)

## उदाहरण

[inline-code-attrs-start title = 'GetAuditLogs उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	limit := float64(1.2) // float64 |  (वैकल्पिक)
	skip := float64(1.2) // float64 |  (वैकल्पिक)
	order := openapiclient.SORT_DIR("ASC") // SORTDIR |  (वैकल्पिक)
	after := float64(1.2) // float64 |  (वैकल्पिक)
	before := float64(1.2) // float64 |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetAuditLogs(context.Background()).TenantId(tenantId).Limit(limit).Skip(skip).Order(order).After(after).Before(before).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetAuditLogs``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetAuditLogs` से प्रतिक्रिया: GetAuditLogsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetAuditLogs`: %v\n", resp)
}
[inline-code-end]