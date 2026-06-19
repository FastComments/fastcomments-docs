Lister sider for en tenant. Bruges af FChat desktop-klienten til at udfylde sin rumliste.
Kræver `enableFChat` at være true på den resolvede custom config for hver side.
Sider, der kræver SSO, filtreres i forhold til den anmodende brugers gruppeadgang.

## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| cursor | string | Nej |  |
| limit | number | Nej |  |
| q | string | Nej |  |
| sortBy | PagesSortBy | Nej |  |
| hasComments | boolean | Nej |  |

## Svar

Returnerer: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f9b2c";
const cursor: string = "cursor_0001a2b3";
const limit: number = 25;
const q: string = "product page";
const hasComments: boolean = true;
const response: GetPublicPagesResponse = await getPagesPublic(tenantId, cursor, limit, q, undefined, hasComments);
[inline-code-end]

---