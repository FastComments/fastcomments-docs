## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| id | string | path | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'updateQuestionResult उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा में हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let updateQuestionResultBody = UpdateQuestionResultBody(urlId: "urlId_example", anonUserId: "anonUserId_example", userId: "userId_example", value: 123, values: [123], commentId: "commentId_example", questionId: "questionId_example", meta: [MetaItem(name: "name_example", values: ["values_example"])]) // UpdateQuestionResultBody | 

DefaultAPI.updateQuestionResult(tenantId: tenantId, id: id, updateQuestionResultBody: updateQuestionResultBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]