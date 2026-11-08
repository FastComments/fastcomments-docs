वेबहुक डिलीवरी में उपयोग होने वाले बिल्कुल उसी रूप में हालिया कमेंट्स, इंटीग्रेशन बनाने के लिए (उदाहरण के लिए Zapier
सैंपल डेटा)। हर इवेंट एक ही कमेंट ऑब्जेक्ट डिलीवर करता है, इसलिए `event` को केवल वैध होना ही ज़रूरी है।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| event | string | query | नहीं |  |
| limit | number | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhookSamplePayloadsResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getWebhookSamplePayloads उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let event = WebhookEventName() // WebhookEventName |  (optional)
let limit = 987 // Double |  (optional)

DefaultAPI.getWebhookSamplePayloads(tenantId: tenantId, options: DefaultAPI.GetWebhookSamplePayloadsOptions(event: event, limit: limit)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]