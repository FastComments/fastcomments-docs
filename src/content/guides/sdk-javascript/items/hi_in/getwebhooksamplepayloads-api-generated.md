वेबहुक डिलीवरी द्वारा उपयोग किए जाने वाले बिल्कुल उसी रूप में हालिया कमेंट्स, इंटीग्रेशन बनाने के लिए (उदाहरण के लिए Zapier
सैंपल डेटा)। हर इवेंट एक ही कमेंट ऑब्जेक्ट डिलीवर करता है, इसलिए `event` केवल वैध होना चाहिए।

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
declare function getWebhookSamplePayloads(
  tenantId: string,
  event?: WebhookEventName,
  limit?: number
): Promise<GetWebhookSamplePayloadsResponse>;

const tenantId: string = '5f8d9b3e2a1b4c6d7e8f9a0b';

// किसी विशिष्ट इवेंट प्रकार के लिए सैंपल पेलोड प्राप्त करें
const commentPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  WebhookEventName.COMMENT_SUCCESS_PAGE,
  10
);

// सभी सैंपल पेलोड प्राप्त करें (कोई वैकल्पिक पैरामीटर नहीं)
const allPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(tenantId);

console.log(`Comment payload status: ${commentPayloads.status}`);
console.log(`Fetched ${allPayloads.payloads.length} sample payloads`);
[inline-code-end]