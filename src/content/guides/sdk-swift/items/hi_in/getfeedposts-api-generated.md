req
tenantId
afterId

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| afterId | string | query | नहीं |  |
| limit | integer | query | नहीं |  |
| tags | array | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetFeedPostsResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getFeedPosts उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (वैकल्पिक)
let limit = 987 // Int |  (वैकल्पिक)
let tags = ["inner_example"] // [String] |  (वैकल्पिक)

DefaultAPI.getFeedPosts(tenantId: tenantId, options: DefaultAPI.GetFeedPostsOptions(afterId: afterId, limit: limit, tags: tags)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]