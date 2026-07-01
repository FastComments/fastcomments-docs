## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| largeInternalURLSanitized | string | Sì |  |

## Risposta

Restituisce: [`GetGifLargeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifLargeResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio getGifLarge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'c9f4a1b2-3d5e-4f6a-8b9c-0d1e2f3a4b5c';
  const largeInternalURLSanitized: string = 'https://cdn.fastcomments.com/gifs/awesome-cat-large.gif';
  const result: GetGifLargeResponse = await getGifLarge(tenantId, largeInternalURLSanitized);
  console.log(result);
})();
[inline-code-end]