Aktuelle Kommentare exakt in der Form, die Webhook‑Auslieferungen verwenden, zum Erstellen von Integrationen (z. B. Zapier‑Beispieldaten). Jedes Ereignis liefert dasselbe Kommentarobjekt, sodass `event` nur gültig sein muss.

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| event | WebhookEventName | Nein |  |
| limit | number | Nein |  |

## Antwort

Rückgabe: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getWebhookSamplePayloads'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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