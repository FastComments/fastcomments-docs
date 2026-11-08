किसी URL को एक कमेंट इवेंट (REST hook subscribe) के लिए सब्सक्राइब करता है। उसी URL को उसी इवेंट और डोमेन के लिए दोबारा सब्सक्राइब करने पर मौजूदा सब्सक्रिप्शन वापस आती है। डिलीवरी HMAC से साइन की जाती हैं, webhooks गाइड देखें; लेगेसी `token` हेडर API सब्सक्रिप्शन को कभी नहीं भेजा जाता।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateWebhookResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'createWebhook उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा में हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createWebhookParams = CreateWebhookParams(url: "url_example", event: WebhookEventName(), domain: "domain_example", method: WebhookHTTPMethod()) // CreateWebhookParams | 

DefaultAPI.createWebhook(tenantId: tenantId, createWebhookParams: createWebhookParams) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]