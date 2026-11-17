Seneste kommentarer i præcis samme format som webhook-leveringer bruger, til at bygge integrationer (for eksempel Zapier
eksempeldata). Hver begivenhed leverer det samme kommentarobjekt, så `event` skal kun være gyldig.

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| event | WebhookEventName | Nej |  |
| limit | number | Nej |  |

## Svar

Returnerer: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getWebhookSamplePayloads Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getWebhookSamplePayloads(
  tenantId: string,
  event?: WebhookEventName,
  limit?: number
): Promise<GetWebhookSamplePayloadsResponse>;

const tenantId: string = '5f8d9b3e2a1b4c6d7e8f9a0b';

// Hent eksempel-payloads for en bestemt begivenhedstype
const commentPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  WebhookEventName.COMMENT_SUCCESS_PAGE,
  10
);

// Hent alle eksempel-payloads (ingen valgfrie parametre)
const allPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(tenantId);

console.log(`Comment payload status: ${commentPayloads.status}`);
console.log(`Hentede ${allPayloads.payloads.length} eksempel-payloads`);
[inline-code-end]