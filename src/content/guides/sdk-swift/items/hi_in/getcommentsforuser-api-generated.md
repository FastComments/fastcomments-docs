## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| userId | string | query | नहीं |  |
| direction | string | query | नहीं |  |
| repliesToUserId | string | query | नहीं |  |
| page | number | query | नहीं |  |
| includei10n | boolean | query | नहीं |  |
| locale | string | query | नहीं |  |
| isCrawler | boolean | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsForUserResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getCommentsForUser उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let userId = "userId_example" // String |  (optional)
let direction = SortDirections() // SortDirections |  (optional)
let repliesToUserId = "repliesToUserId_example" // String |  (optional)
let page = 987 // Double |  (optional)
let includei10n = true // Bool |  (optional)
let locale = "locale_example" // String |  (optional)
let isCrawler = true // Bool |  (optional)

PublicAPI.getCommentsForUser(options: PublicAPI.GetCommentsForUserOptions(userId: userId, direction: direction, repliesToUserId: repliesToUserId, page: page, includei10n: includei10n, locale: locale, isCrawler: isCrawler)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]