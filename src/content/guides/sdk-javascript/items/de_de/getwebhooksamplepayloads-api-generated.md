Neueste Kommentare in genau der Form, wie sie bei Webhook-Zustellungen verwendet werden, zum Aufbau von Integrationen (zum Beispiel Zapier-Beispieldaten). Jedes Ereignis liefert dasselbe Kommentarobjekt, daher muss `event` nur gültig sein.

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| event | WebhookEventName | Nein |  |
| limit | number | Nein |  |

## Antwort

Gibt zurück: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getWebhookSamplePayloads Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getWebhookSamplePayloads(
  tenantId: string,
  event?: WebhookEventName,
  limit?: number
): Promise<GetWebhookSamplePayloadsResponse>;

const tenantId: string = '5f8d9b3e2a1b4c6d7e8f9a0b';

// Get sample payloads for a specific event type
const commentPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  WebhookEventName.COMMENT_SUCCESS_PAGE,
  10
);

// Get all sample payloads (no optional parameters)
const allPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(tenantId);

console.log(`Comment payload status: ${commentPayloads.status}`);
console.log(`Fetched ${allPayloads.payloads.length} sample payloads`);
[inline-code-end]