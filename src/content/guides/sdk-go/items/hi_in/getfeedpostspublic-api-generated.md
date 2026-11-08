req
tenantId
afterId

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| afterId | string | query | नहीं |  |
| limit | integer | query | नहीं |  |
| tags | array | query | नहीं |  |
| sso | string | query | नहीं |  |
| isCrawler | boolean | query | नहीं |  |
| includeUserInfo | boolean | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_public_feed_posts_response.go)

## उदाहरण

[inline-code-attrs-start title = 'GetFeedPostsPublic उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	limit := int32(56) // int32 |  (वैकल्पिक)
	tags := []string{"Inner_example"} // []string |  (वैकल्पिक)
	sso := "sso_example" // string |  (वैकल्पिक)
	isCrawler := true // bool |  (वैकल्पिक)
	includeUserInfo := true // bool |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetFeedPostsPublic(context.Background(), tenantId).AfterId(afterId).Limit(limit).Tags(tags).Sso(sso).IsCrawler(isCrawler).IncludeUserInfo(includeUserInfo).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetFeedPostsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetFeedPostsPublic`: PublicFeedPostsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetFeedPostsPublic`: %v\n", resp)
}
[inline-code-end]