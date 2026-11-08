## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| page | integer | query | नहीं |  |
| limit | integer | query | नहीं |  |
| skip | integer | query | नहीं |  |
| asTree | boolean | query | नहीं |  |
| skipChildren | integer | query | नहीं |  |
| limitChildren | integer | query | नहीं |  |
| maxTreeDepth | integer | query | नहीं |  |
| urlId | string | query | नहीं |  |
| userId | string | query | नहीं |  |
| anonUserId | string | query | नहीं |  |
| contextUserId | string | query | नहीं |  |
| hashTag | string | query | नहीं |  |
| parentId | string | query | नहीं |  |
| direction | string | query | नहीं |  |
| fromDate | integer | query | नहीं |  |
| toDate | integer | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetCommentsResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getComments उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा में हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (वैकल्पिक)
let limit = 987 // Int |  (वैकल्पिक)
let skip = 987 // Int |  (वैकल्पिक)
let asTree = true // Bool |  (वैकल्पिक)
let skipChildren = 987 // Int |  (वैकल्पिक)
let limitChildren = 987 // Int |  (वैकल्पिक)
let maxTreeDepth = 987 // Int |  (वैकल्पिक)
let urlId = "urlId_example" // String |  (वैकल्पिक)
let userId = "userId_example" // String |  (वैकल्पिक)
let anonUserId = "anonUserId_example" // String |  (वैकल्पिक)
let contextUserId = "contextUserId_example" // String |  (वैकल्पिक)
let hashTag = "hashTag_example" // String |  (वैकल्पिक)
let parentId = "parentId_example" // String |  (वैकल्पिक)
let direction = SortDirections() // SortDirections |  (वैकल्पिक)
let fromDate = 987 // Int64 |  (वैकल्पिक)
let toDate = 987 // Int64 |  (वैकल्पिक)

DefaultAPI.getComments(tenantId: tenantId, options: DefaultAPI.GetCommentsOptions(page: page, limit: limit, skip: skip, asTree: asTree, skipChildren: skipChildren, limitChildren: limitChildren, maxTreeDepth: maxTreeDepth, urlId: urlId, userId: userId, anonUserId: anonUserId, contextUserId: contextUserId, hashTag: hashTag, parentId: parentId, direction: direction, fromDate: fromDate, toDate: toDate)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]