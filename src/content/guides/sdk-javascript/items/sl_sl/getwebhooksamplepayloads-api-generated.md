Nedavni komentarji v točno takšni obliki, kot jo uporabljajo dostave webhookov, primarni za izdelavo integracij (na primer vzorčni podatki za
Zapier). Vsak dogodek dostavi isti objekt komentarja, zato mora biti `event` le veljaven.

## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| event | WebhookEventName | Ne |  |
| limit | number | Ne |  |

## Odgovor

Vrne: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getWebhookSamplePayloads'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getWebhookSamplePayloads(
  tenantId: string,
  event?: WebhookEventName,
  limit?: number
): Promise<GetWebhookSamplePayloadsResponse>;

const tenantId: string = '5f8d9b3e2a1b4c6d7e8f9a0b';

// Pridobi vzorčne podatke za določeno vrsto dogodka
const commentPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  WebhookEventName.COMMENT_SUCCESS_PAGE,
  10
);

// Pridobi vse vzorčne podatke (brez neobveznih parametrov)
const allPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(tenantId);

console.log(`Stanje podatkov komentarja: ${commentPayloads.status}`);
console.log(`Pridobljenih ${allPayloads.payloads.length} vzorčnih podatkov`);
[inline-code-end]