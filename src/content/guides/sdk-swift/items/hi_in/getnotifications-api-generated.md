## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| userId | string | query | नहीं |  |
| urlId | string | query | नहीं |  |
| fromCommentId | string | query | नहीं |  |
| viewed | boolean | query | नहीं |  |
| type | string | query | नहीं |  |
| skip | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetNotificationsResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getNotifications उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new के माध्यम से रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (वैकल्पिक)
let urlId = "urlId_example" // String |  (वैकल्पिक)
let fromCommentId = "fromCommentId_example" // String |  (वैकल्पिक)
let viewed = true // Bool |  (वैकल्पिक)
let type = "type_example" // String |  (वैकल्पिक)
let skip = 987 // Double |  (वैकल्पिक)

DefaultAPI.getNotifications(tenantId: tenantId, options: DefaultAPI.GetNotificationsOptions(userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]