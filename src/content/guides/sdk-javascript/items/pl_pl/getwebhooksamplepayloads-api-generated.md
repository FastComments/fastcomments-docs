Ostatnie komentarze w dokładnie takiej postaci, jakiej używają dostarczenia webhooków, przydatne do budowania integracji (na przykład przykładowe dane dla Zapier). Każde zdarzenie dostarcza ten sam obiekt komentarza, więc `event` musi być jedynie prawidłowy.

## Parametry

| Nazwa | Typ | Wymagany | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| event | WebhookEventName | Nie |  |
| limit | number | Nie |  |

## Odpowiedź

Zwraca: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getWebhookSamplePayloads'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getWebhookSamplePayloads(
  tenantId: string,
  event?: WebhookEventName,
  limit?: number
): Promise<GetWebhookSamplePayloadsResponse>;

const tenantId: string = '5f8d9b3e2a1b4c6d7e8f9a0b';

// Pobierz przykładowe ładunki dla określonego typu zdarzenia
const commentPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  WebhookEventName.COMMENT_SUCCESS_PAGE,
  10
);

// Pobierz wszystkie przykładowe ładunki (bez parametrów opcjonalnych)
const allPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(tenantId);

console.log(`Comment payload status: ${commentPayloads.status}`);
console.log(`Fetched ${allPayloads.payloads.length} sample payloads`);
[inline-code-end]