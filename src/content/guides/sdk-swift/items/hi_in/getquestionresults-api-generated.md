## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| urlId | string | query | नहीं |  |
| userId | string | query | नहीं |  |
| startDate | string | query | नहीं |  |
| questionId | string | query | नहीं |  |
| questionIds | string | query | नहीं |  |
| skip | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetQuestionResultsResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getQuestionResults उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा में हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String |  (वैकल्पिक)
let userId = "userId_example" // String |  (वैकल्पिक)
let startDate = "startDate_example" // String |  (वैकल्पिक)
let questionId = "questionId_example" // String |  (वैकल्पिक)
let questionIds = "questionIds_example" // String |  (वैकल्पिक)
let skip = 987 // Double |  (वैकल्पिक)

DefaultAPI.getQuestionResults(tenantId: tenantId, options: DefaultAPI.GetQuestionResultsOptions(urlId: urlId, userId: userId, startDate: startDate, questionId: questionId, questionIds: questionIds, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]