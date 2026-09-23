List sider for en lejer. Bruges af FChat desktop‑klienten til at udfylde sin rumliste.  
Kræver `enableFChat` at være sand på den løste brugerdefinerede konfiguration for hver side.  
Sider, der kræver SSO, filtreres i forhold til den anmodende brugers gruppeadgang.

## Parameters

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| cursor | string | Nej |  |
| limit | number | Nej |  |
| q | string | Nej |  |
| sortBy | PagesSortBy | Nej |  |
| hasComments | boolean | Nej |  |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Example

[inline-code-attrs-start title = 'getPagesPublic Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPublicPages() {
  const tenantId: string = "tenant_12345";
  const cursor: string = "page_5";
  const limit: number = 20;
  const query: string = "support";
  const hasComments: boolean = true;

  const response: GetPublicPagesResponse = await getPagesPublic(
    tenantId,
    cursor,
    limit,
    query,
    undefined,
    hasComments
  );

  console.log(response);
}
[inline-code-end]

---