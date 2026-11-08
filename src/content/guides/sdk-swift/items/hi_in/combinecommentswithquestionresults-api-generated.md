## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| questionId | string | query | नहीं |  |
| questionIds | array | query | नहीं |  |
| urlId | string | query | नहीं |  |
| startDate | string | query | नहीं |  |
| forceRecalculate | boolean | query | नहीं |  |
| minValue | number | query | नहीं |  |
| maxValue | number | query | नहीं |  |
| limit | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CombineQuestionResultsWithCommentsResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा में हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (वैकल्पिक)
let questionIds = ["inner_example"] // [String] |  (वैकल्पिक)
let urlId = "urlId_example" // String |  (वैकल्पिक)
let startDate = Date() // Date |  (वैकल्पिक)
let forceRecalculate = true // Bool |  (वैकल्पिक)
let minValue = 987 // Double |  (वैकल्पिक)
let maxValue = 987 // Double |  (वैकल्पिक)
let limit = 987 // Double |  (वैकल्पिक)

DefaultAPI.combineCommentsWithQuestionResults(tenantId: tenantId, options: DefaultAPI.CombineCommentsWithQuestionResultsOptions(questionId: questionId, questionIds: questionIds, urlId: urlId, startDate: startDate, forceRecalculate: forceRecalculate, minValue: minValue, maxValue: maxValue, limit: limit)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]