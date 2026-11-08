## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| postId | string | path | हाँ |  |
| broadcastId | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_feed_post_response.go)

## उदाहरण

[inline-code-attrs-start title = 'UpdateFeedPostPublic उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	postId := "postId_example" // string | 
	updateFeedPostParams := *openapiclient.NewUpdateFeedPostParams() // UpdateFeedPostParams | 
	broadcastId := "broadcastId_example" // string |  (वैकल्पिक)
	sso := "sso_example" // string |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UpdateFeedPostPublic(context.Background(), tenantId, postId).UpdateFeedPostParams(updateFeedPostParams).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "`PublicAPI.UpdateFeedPostPublic` को कॉल करते समय त्रुटि: %v\n", err)
		fmt.Fprintf(os.Stderr, "पूर्ण HTTP प्रतिक्रिया: %v\n", r)
	}
	// `UpdateFeedPostPublic` से प्रतिक्रिया: CreateFeedPostResponse
	fmt.Fprintf(os.Stdout, "`PublicAPI.UpdateFeedPostPublic` से प्रतिक्रिया: %v\n", resp)
}
[inline-code-end]