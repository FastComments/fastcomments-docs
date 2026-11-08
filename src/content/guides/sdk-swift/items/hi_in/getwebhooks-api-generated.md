टेनेंट के लिए कॉन्फ़िगर किए गए वेबहुक्स की सूची देता है, जिसमें डैशबोर्ड-प्रबंधित पंक्तियाँ और API सब्सक्रिप्शन दोनों शामिल हैं।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| event | string | query | नहीं |  |
| domain | string | query | नहीं |  |
| source | string | query | नहीं |  |
| skip | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhooksResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getWebhooks उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा में हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let event = WebhookEventName() // WebhookEventName |  (वैकल्पिक)
let domain = "domain_example" // String |  (वैकल्पिक)
let source = WebhookSource() // WebhookSource |  (वैकल्पिक)
let skip = 987 // Double |  (वैकल्पिक)

DefaultAPI.getWebhooks(tenantId: tenantId, options: DefaultAPI.GetWebhooksOptions(event: event, domain: domain, source: source, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]