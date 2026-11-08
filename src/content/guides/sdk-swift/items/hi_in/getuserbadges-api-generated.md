## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| userId | string | query | नहीं |  |
| badgeId | string | query | नहीं |  |
| type | number | query | नहीं |  |
| displayedOnComments | boolean | query | नहीं |  |
| limit | number | query | नहीं |  |
| skip | number | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetUserBadgesResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getUserBadges उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा में हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new के माध्यम से रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (वैकल्पिक)
let badgeId = "badgeId_example" // String |  (वैकल्पिक)
let type = 987 // Double |  (वैकल्पिक)
let displayedOnComments = true // Bool |  (वैकल्पिक)
let limit = 987 // Double |  (वैकल्पिक)
let skip = 987 // Double |  (वैकल्पिक)

DefaultAPI.getUserBadges(tenantId: tenantId, options: DefaultAPI.GetUserBadgesOptions(userId: userId, badgeId: badgeId, type: type, displayedOnComments: displayedOnComments, limit: limit, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]