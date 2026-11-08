अनसब्सक्राइब (REST hook unsubscribe)। केवल इस API के माध्यम से बनाए गए सब्सक्रिप्शन यहाँ से डिलीट किए जा सकते हैं; डैशबोर्ड-प्रबंधित वेबहुक डैशबोर्ड में एडिट किए जाते हैं।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'deleteWebhook उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const webhookId: string = "wh_98765";

const result: APIEmptyResponse = await deleteWebhook(tenantId, webhookId);
[inline-code-end]