req
tenantId
afterId

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| afterId | string | query | नहीं |  |
| limit | integer | query | नहीं |  |
| tags | array | query | नहीं |  |
| sso | string | query | नहीं |  |
| isCrawler | boolean | query | नहीं |  |
| includeUserInfo | boolean | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PublicFeedPostsResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getFeedPostsPublic उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा में हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (optional)
let limit = 987 // Int |  (optional)
let tags = ["inner_example"] // [String] |  (optional)
let sso = "sso_example" // String |  (optional)
let isCrawler = true // Bool |  (optional)
let includeUserInfo = true // Bool |  (optional)

PublicAPI.getFeedPostsPublic(tenantId: tenantId, options: PublicAPI.GetFeedPostsPublicOptions(afterId: afterId, limit: limit, tags: tags, sso: sso, isCrawler: isCrawler, includeUserInfo: includeUserInfo)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]