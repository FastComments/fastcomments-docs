## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| largeInternalURLSanitized | string | Ja |  |

## Respons

Retourneert: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifGetLargeResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getGifLarge Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme_marketing_tenant_7';
const largeInternalURLSanitized: string = 'https://cdn.acmeinc.com/gifs/promo-spring-2026_large_sanitized.gif';
const includePreview: boolean | undefined = undefined; // optionele vlag die een aanroeper mogelijk gebruikt
const result: GifGetLargeResponse = await getGifLarge(tenantId, largeInternalURLSanitized);
console.log(result, includePreview);
[inline-code-end]

---