## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| commentId | string | query | नहीं |  |
| externalId | string | query | नहीं |  |
| eventType | string | query | नहीं |  |
| type | string | query | नहीं |  |
| domain | string | query | नहीं |  |
| attemptCountGT | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEventCountResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getPendingWebhookEventCount उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String |  (वैकल्पिक)
let externalId = "externalId_example" // String |  (वैकल्पिक)
let eventType = "eventType_example" // String |  (वैकल्पिक)
let type = "type_example" // String |  (वैकल्पिक)
let domain = "domain_example" // String |  (वैकल्पिक)
let attemptCountGT = 987 // Double |  (वैकल्पिक)

DefaultAPI.getPendingWebhookEventCount(tenantId: tenantId, options: DefaultAPI.GetPendingWebhookEventCountOptions(commentId: commentId, externalId: externalId, eventType: eventType, type: type, domain: domain, attemptCountGT: attemptCountGT)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]