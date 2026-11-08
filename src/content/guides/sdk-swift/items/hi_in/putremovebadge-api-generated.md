## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| badgeId | string | query | हाँ |  |
| userId | string | query | नहीं |  |
| commentId | string | query | नहीं |  |
| broadcastId | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/RemoveUserBadgeResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'putRemoveBadge उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let badgeId = "badgeId_example" // String | 
let userId = "userId_example" // String |  (वैकल्पिक)
let commentId = "commentId_example" // String |  (वैकल्पिक)
let broadcastId = "broadcastId_example" // String |  (वैकल्पिक)
let sso = "sso_example" // String |  (वैकल्पिक)

ModerationAPI.putRemoveBadge(tenantId: tenantId, badgeId: badgeId, options: ModerationAPI.PutRemoveBadgeOptions(userId: userId, commentId: commentId, broadcastId: broadcastId, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]