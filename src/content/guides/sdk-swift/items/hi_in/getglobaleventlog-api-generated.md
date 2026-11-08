req
tenantId
urlId
userIdWS

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| userIdWS | string | query | हाँ |  |
| startTime | integer | query | हाँ |  |
| endTime | integer | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetEventLogResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getGlobalEventLog उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा में हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let userIdWS = "userIdWS_example" // String | 
let startTime = 987 // Int64 | 
let endTime = 987 // Int64 |  (वैकल्पिक)

PublicAPI.getGlobalEventLog(tenantId: tenantId, urlId: urlId, userIdWS: userIdWS, startTime: startTime, endTime: endTime) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]