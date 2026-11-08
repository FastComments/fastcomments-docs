## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| postIds | array | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UserReactsResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getUserReactsPublic उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा में हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let postIds = ["inner_example"] // [String] |  (वैकल्पिक)
let sso = "sso_example" // String |  (वैकल्पिक)

PublicAPI.getUserReactsPublic(tenantId: tenantId, options: PublicAPI.GetUserReactsPublicOptions(postIds: postIds, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]