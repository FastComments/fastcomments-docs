## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| usernameStartsWith | string | query | नहीं |  |
| mentionGroupIds | array | query | नहीं |  |
| sso | string | query | नहीं |  |
| searchSection | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SearchUsersResult.swift)

## उदाहरण

[inline-code-attrs-start title = 'searchUsers उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा में हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let usernameStartsWith = "usernameStartsWith_example" // String |  (वैकल्पिक)
let mentionGroupIds = ["inner_example"] // [String] |  (वैकल्पिक)
let sso = "sso_example" // String |  (वैकल्पिक)
let searchSection = "searchSection_example" // String |  (वैकल्पिक)

PublicAPI.searchUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.SearchUsersOptions(usernameStartsWith: usernameStartsWith, mentionGroupIds: mentionGroupIds, sso: sso, searchSection: searchSection)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]