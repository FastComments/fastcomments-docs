## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| page | integer | query | नहीं |  |
| limit | integer | query | नहीं |  |
| skip | integer | query | नहीं |  |
| asTree | boolean | query | नहीं |  |
| skipChildren | integer | query | नहीं |  |
| limitChildren | integer | query | नहीं |  |
| maxTreeDepth | integer | query | नहीं |  |
| urlId | string | query | नहीं |  |
| userId | string | query | नहीं |  |
| anonUserId | string | query | नहीं |  |
| contextUserId | string | query | नहीं |  |
| hashTag | string | query | नहीं |  |
| parentId | string | query | नहीं |  |
| direction | string | query | नहीं |  |
| fromDate | integer | query | नहीं |  |
| toDate | integer | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_get_comments_response.go)

## उदाहरण

[inline-code-attrs-start title = 'GetComments उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (वैकल्पिक)
	limit := int32(56) // int32 |  (वैकल्पिक)
	skip := int32(56) // int32 |  (वैकल्पिक)
	asTree := true // bool |  (वैकल्पिक)
	skipChildren := int32(56) // int32 |  (वैकल्पिक)
	limitChildren := int32(56) // int32 |  (वैकल्पिक)
	maxTreeDepth := int32(56) // int32 |  (वैकल्पिक)
	urlId := "urlId_example" // string |  (वैकल्पिक)
	userId := "userId_example" // string |  (वैकल्पिक)
	anonUserId := "anonUserId_example" // string |  (वैकल्पिक)
	contextUserId := "contextUserId_example" // string |  (वैकल्पिक)
	hashTag := "hashTag_example" // string |  (वैकल्पिक)
	parentId := "parentId_example" // string |  (वैकल्पिक)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (वैकल्पिक)
	fromDate := int64(789) // int64 |  (वैकल्पिक)
	toDate := int64(789) // int64 |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetComments(context.Background()).TenantId(tenantId).Page(page).Limit(limit).Skip(skip).AsTree(asTree).SkipChildren(skipChildren).LimitChildren(limitChildren).MaxTreeDepth(maxTreeDepth).UrlId(urlId).UserId(userId).AnonUserId(anonUserId).ContextUserId(contextUserId).HashTag(hashTag).ParentId(parentId).Direction(direction).FromDate(fromDate).ToDate(toDate).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetComments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetComments` से प्रतिक्रिया: APIGetCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetComments`: %v\n", resp)
}
[inline-code-end]