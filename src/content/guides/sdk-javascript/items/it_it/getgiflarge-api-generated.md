## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| largeInternalURLSanitized | string | Sì |  |

## Risposta

Restituisce: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifGetLargeResponse.ts)

## Esempio

[inline-code-attrs-start title = 'getGifLarge Esempio'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
    const tenantId: string = "c1a2b3d4-5678-90ab-cdef-1234567890ab";
    const largeInternalURLSanitized: string = "https://cdn.fastcomments.com/gifs/large/abc123.gif";
    const response: GifGetLargeResponse = await getGifLarge(tenantId, largeInternalURLSanitized);
    console.log(response);
}
[inline-code-end]