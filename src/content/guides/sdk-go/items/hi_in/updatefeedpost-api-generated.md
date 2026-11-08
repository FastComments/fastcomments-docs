## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| id | string | path | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_empty_response.go)

## उदाहरण

[inline-code-attrs-start title = 'UpdateFeedPost उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
    "time"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	id := "id_example" // string | 
	feedPost := *openapiclient.NewFeedPost("Id_example", "TenantId_example", time.Now()) // FeedPost | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.UpdateFeedPost(context.Background(), id).TenantId(tenantId).FeedPost(feedPost).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.UpdateFeedPost``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `UpdateFeedPost` से प्रतिक्रिया: APIEmptyResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.UpdateFeedPost`: %v\n", resp)
}
[inline-code-end]