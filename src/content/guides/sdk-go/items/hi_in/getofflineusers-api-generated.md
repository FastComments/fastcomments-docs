पेज पर उन पिछले टिप्पणीकारों के बारे में जो अभी ऑनलाइन नहीं हैं। displayName के अनुसार क्रमबद्ध।
/users/online का उपयोग पूरा करने के बाद "Members" सेक्शन रेंडर करने के लिए इसका उपयोग करें।
commenterName पर कर्सर पेजिनेशन: सर्वर afterName से आगे $gt के माध्यम से आंशिक {tenantId, urlId, commenterName}
इंडेक्स को बिना $skip लागत के चलता है।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ | पेज URL पहचानकर्ता (सर्वर-साइड पर साफ़ किया गया)। |
| afterName | string | query | नहीं | कर्सर: पिछली प्रतिक्रिया से nextAfterName पास करें। |
| afterUserId | string | query | नहीं | कर्सर टाइब्रेकर: पिछली प्रतिक्रिया से nextAfterUserId पास करें। आवश्यक है जब afterName सेट हो ताकि समान नाम वाली प्रविष्टियाँ न छूटें। |

## प्रतिक्रिया

लौटाता है: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_offline_response.go)

## उदाहरण

[inline-code-attrs-start title = 'GetOfflineUsers उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | पेज URL पहचानकर्ता (सर्वर-साइड पर साफ़ किया गया)।
	afterName := "afterName_example" // string | कर्सर: पिछली प्रतिक्रिया से nextAfterName पास करें। (optional)
	afterUserId := "afterUserId_example" // string | कर्सर टाइब्रेकर: पिछली प्रतिक्रिया से nextAfterUserId पास करें। आवश्यक है जब afterName सेट हो ताकि समान नाम वाली प्रविष्टियाँ न छूटें। (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOfflineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOfflineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetOfflineUsers`: PageUsersOfflineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOfflineUsers`: %v\n", resp)
}
[inline-code-end]