## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | No |  |

## תגובה

מחזיר: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_add_hash_tag_200_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-AddHashTag'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string |  (אופציונלי)
	createHashTagBody := *openapiclient.NewCreateHashTagBody("Tag_example") // CreateHashTagBody |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.AddHashTag(context.Background()).TenantId(tenantId).CreateHashTagBody(createHashTagBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.AddHashTag``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תשובה מ-`AddHashTag`: AddHashTag200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.AddHashTag`: %v\n", resp)
}
[inline-code-end]

---