Commenti recenti esattamente nel formato utilizzato dalle consegne webhook, per creare integrazioni (ad esempio dati di esempio
Zapier). Ogni evento consegna lo stesso oggetto commento, quindi `event` deve solo essere valido.

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| event | WebhookEventName | No |  |
| limit | number | No |  |

## Risposta

Restituisce: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getWebhookSamplePayloads'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getWebhookSamplePayloads(
  tenantId: string,
  event?: WebhookEventName,
  limit?: number
): Promise<GetWebhookSamplePayloadsResponse>;

const tenantId: string = '5f8d9b3e2a1b4c6d7e8f9a0b';

// Ottiene i payload di esempio per un tipo di evento specifico
const commentPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  WebhookEventName.COMMENT_SUCCESS_PAGE,
  10
);

// Ottiene tutti i payload di esempio (nessun parametro opzionale)
const allPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(tenantId);

console.log(`Comment payload status: ${commentPayloads.status}`);
console.log(`Fetched ${allPayloads.payloads.length} sample payloads`);
[inline-code-end]