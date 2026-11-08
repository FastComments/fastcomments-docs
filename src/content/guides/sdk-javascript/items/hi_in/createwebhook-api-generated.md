किसी URL को एक कमेंट इवेंट (REST hook subscribe) के लिए सब्सक्राइब करता है। उसी URL को उसी इवेंट और डोमेन के लिए फिर से सब्सक्राइब करने पर मौजूदा सब्सक्रिप्शन वापस आती है। डिलीवरी HMAC से साइन की जाती हैं, webhooks गाइड देखें; लेगेसी `token` हेडर कभी भी API सब्सक्रिप्शन को नहीं भेजा जाता।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| createWebhookParams | CreateWebhookParams | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateWebhookResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'createWebhook उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const webhookParams: CreateWebhookParams = {
  event: "comment.created" as WebhookEventName,
  method: "POST" as WebhookHTTPMethod,
  url: "https://myapp.example.com/webhook",
  source: "comment" as WebhookSource,
  // पेलोड सत्यापन के लिए वैकल्पिक सीक्रेट
  secret: "s3cr3tK3y123",
};

const result: CreateWebhookResponse = await createWebhook(tenantId, webhookParams);

const webhookId: string = result.webhook.id;
[inline-code-end]