## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| page | number | query | नहीं |  |
| count | number | query | नहीं |  |
| text-search | string | query | नहीं |  |
| byIPFromComment | string | query | नहीं |  |
| filters | string | query | नहीं |  |
| searchFilters | string | query | नहीं |  |
| sorts | string | query | नहीं |  |
| demo | boolean | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentsResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getApiComments उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा में हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new के माध्यम से रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Double |  (वैकल्पिक)
let count = 987 // Double |  (वैकल्पिक)
let textSearch = "textSearch_example" // String |  (वैकल्पिक)
let byIPFromComment = "byIPFromComment_example" // String |  (वैकल्पिक)
let filters = "filters_example" // String |  (वैकल्पिक)
let searchFilters = "searchFilters_example" // String |  (वैकल्पिक)
let sorts = "sorts_example" // String |  (वैकल्पिक)
let demo = true // Bool |  (वैकल्पिक)
let sso = "sso_example" // String |  (वैकल्पिक)

ModerationAPI.getApiComments(tenantId: tenantId, options: ModerationAPI.GetApiCommentsOptions(page: page, count: count, textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, sorts: sorts, demo: demo, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]