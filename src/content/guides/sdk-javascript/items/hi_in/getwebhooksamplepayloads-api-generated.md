वेबहुक डिलीवरी द्वारा उपयोग किए जाने वाले बिल्कुल उसी रूप में हाल की टिप्पणियाँ, इंटीग्रेशन बनाने के लिए (उदाहरण के लिए Zapier
सैंपल डेटा)। हर इवेंट एक ही कमेंट ऑब्जेक्ट डिलीवर करता है, इसलिए `event` को केवल वैध होना ही ज़रूरी है।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| event | WebhookEventName | नहीं |  |
| limit | number | नहीं |  |

## प्रतिक्रिया

रिटर्न: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getWebhookSamplePayloads उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c1a2b3d4-5678-90ab-cdef-1234567890ab";

const responseAll: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  "comment.created",
  10
);

const responseWithEvent: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  "comment.created"
);

const responseBasic: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId
);
[inline-code-end]