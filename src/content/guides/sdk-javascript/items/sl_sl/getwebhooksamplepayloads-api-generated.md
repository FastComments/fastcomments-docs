Nedavni komentarji v točno isti obliki, ki jo uporabljajo dostave webhookov, za gradnjo integracij (na primer vzorčni podatki Zapier). Vsak dogodek pošlje isti objekt komentarja, zato mora biti `event` le veljaven.

## Parameters

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| event | WebhookEventName | Ne |  |
| limit | number | Ne |  |

## Odgovor

Vrne: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## Primer

[inline-code-attrs-start title = 'getWebhookSamplePayloads Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---