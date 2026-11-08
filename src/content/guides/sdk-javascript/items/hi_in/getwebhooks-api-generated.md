टेनेंट के लिए कॉन्फ़िगर किए गए वेबहुक्स की सूची देता है, जिसमें डैशबोर्ड-प्रबंधित पंक्तियाँ और API सब्सक्रिप्शन दोनों शामिल हैं।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| event | WebhookEventName | नहीं |  |
| domain | string | नहीं |  |
| source | WebhookSource | नहीं |  |
| skip | number | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhooksResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getWebhooks उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_42';
  const event: WebhookEventName = 'comment.created';
  const domain: string = 'forum.example.org';
  const source: WebhookSource = 'admin';
  const skip: number = 0;

  const fullResponse: GetWebhooksResponse = await getWebhooks(tenantId, event, domain, source, skip);
  const minimalResponse: GetWebhooksResponse = await getWebhooks(tenantId);
})();
[inline-code-end]