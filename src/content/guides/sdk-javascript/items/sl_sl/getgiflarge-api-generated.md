## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| largeInternalURLSanitized | string | Da |  |

## Odgovor

Vrne: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifGetLargeResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer uporabe getGifLarge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme_marketing_tenant_7';
const largeInternalURLSanitized: string = 'https://cdn.acmeinc.com/gifs/promo-spring-2026_large_sanitized.gif';
const includePreview: boolean | undefined = undefined; // neobvezna zastavica, ki jo lahko uporabi klicatelj
const result: GifGetLargeResponse = await getGifLarge(tenantId, largeInternalURLSanitized);
console.log(result, includePreview);
[inline-code-end]

---