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

रिटर्न करता है: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMyNotificationsResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getUserNotifications उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | यह निर्धारित करने के लिए उपयोग किया जाता है कि क्या वर्तमान पृष्ठ सब्सक्राइब है। (optional)
let pageSize = 987 // Int |  (optional)
let afterId = "afterId_example" // String |  (optional)
let includeContext = true // Bool |  (optional)
let afterCreatedAt = 987 // Int64 |  (optional)
let unreadOnly = true // Bool |  (optional)
let dmOnly = true // Bool |  (optional)
let noDm = true // Bool |  (optional)
let includeTranslations = true // Bool |  (optional)
let includeTenantNotifications = true // Bool |  (optional)
let sso = "sso_example" // String |  (optional)

PublicAPI.getUserNotifications(tenantId: tenantId, options: PublicAPI.GetUserNotificationsOptions(urlId: urlId, pageSize: pageSize, afterId: afterId, includeContext: includeContext, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, includeTranslations: includeTranslations, includeTenantNotifications: includeTenantNotifications, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]