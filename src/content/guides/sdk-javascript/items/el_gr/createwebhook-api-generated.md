Συνδέει ένα URL σε ένα συμβάν σχολίου (REST hook subscribe). Η επανασύνδεση του ίδιου URL στο ίδιο
συμβάν και domain επιστρέφει την υπάρχουσα συνδρομή. Οι παραδόσεις υπογράφονται με HMAC, δείτε τον
οδηγό webhooks· η παλαιού τύπου κεφαλίδα `token` δεν αποστέλλεται ποτέ σε συνδρομές μέσω API.

## Παράμετροι

| Όνομα | Τύπος | Απαραίτητο | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| createWebhookParams | CreateWebhookParams | Ναι |  |

## Απάντηση

Επιστρέφει: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateWebhookResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createWebhook'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const webhookParams: CreateWebhookParams = {
  event: "comment.created" as WebhookEventName,
  method: "POST" as WebhookHTTPMethod,
  url: "https://myapp.example.com/webhook",
  source: "comment" as WebhookSource,
  // προαιρετικό μυστικό για επαλήθευση του payload
  secret: "s3cr3tK3y123",
};

const result: CreateWebhookResponse = await createWebhook(tenantId, webhookParams);

const webhookId: string = result.webhook.id;
[inline-code-end]