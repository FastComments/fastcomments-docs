## Parametry

| Nazwa | Typ | Wymagany | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| urlId | string | Tak |  |
| sso | string | Nie |  |
| includeUserInfo | PageReactIncludeUserInfo | Nie |  |

## Odpowiedź

Zwraca: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReacts.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getV2PageReacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc1234567890';
const urlId: string = '/blog/my-article';

const result: GetV2PageReacts = await getV2PageReacts(tenantId, urlId, 'sso-token-value', 'basic');
[inline-code-end]