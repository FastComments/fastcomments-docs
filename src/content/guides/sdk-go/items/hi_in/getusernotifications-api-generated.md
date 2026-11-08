## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| urlId | string | query | नहीं | यह निर्धारित करने के लिए उपयोग किया जाता है कि क्या वर्तमान पृष्ठ सब्सक्राइब है। |
| pageSize | integer | query | नहीं |  |
| afterId | string | query | नहीं |  |
| includeContext | boolean | query | नहीं |  |
| afterCreatedAt | integer | query | नहीं |  |
| unreadOnly | boolean | query | नहीं |  |
| dmOnly | boolean | query | नहीं |  |
| noDm | boolean | query | नहीं |  |
| includeTranslations | boolean | query | नहीं |  |
| includeTenantNotifications | boolean | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_my_notifications_response.go)

## उदाहरण

[inline-code-attrs-start title = 'GetUserNotifications उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | यह निर्धारित करने के लिए उपयोग किया जाता है कि क्या वर्तमान पृष्ठ सब्सक्राइब है। (optional)
	pageSize := int32(56) // int32 |  (optional)
	afterId := "afterId_example" // string |  (optional)
	includeContext := true // bool |  (optional)
	afterCreatedAt := int64(789) // int64 |  (optional)
	unreadOnly := true // bool |  (optional)
	dmOnly := true // bool |  (optional)
	noDm := true // bool |  (optional)
	includeTranslations := true // bool |  (optional)
	includeTenantNotifications := true // bool |  (optional)
	sso := "sso_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserNotifications(context.Background()).TenantId(tenantId).UrlId(urlId).PageSize(pageSize).AfterId(afterId).IncludeContext(includeContext).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).IncludeTranslations(includeTranslations).IncludeTenantNotifications(includeTenantNotifications).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetUserNotifications`: GetMyNotificationsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserNotifications`: %v\n", resp)
}
[inline-code-end]