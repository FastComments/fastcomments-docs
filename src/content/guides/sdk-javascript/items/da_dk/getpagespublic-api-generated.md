List sider for en lejer. Bruges af FChat desktopklienten til at udfylde dens rumliste.  
Kræver at `enableFChat` er true på den løste brugerdefinerede konfiguration for hver side.  
Sider, der kræver SSO, filtreres i henhold til den anmodende brugers gruppeadgang.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| cursor | string | No |  |
| limit | number | No |  |
| q | string | No |  |
| sortBy | PagesSortBy | No |  |
| hasComments | boolean | No |  |

## Response

Returnerer: [`GetPagesPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublicResponse.ts)

## Example

[inline-code-attrs-start title = 'getPagesPublic Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPages() {
  const tenantId: string = "tenant_12345";
  const cursor: string = "nextPageToken";
  const limit: number = 20;
  const q: string = "blog";
  const sortBy: PagesSortBy = "createdAt";
  const hasComments: boolean = true;

  const response: GetPagesPublicResponse = await getPagesPublic(
    tenantId,
    cursor,
    limit,
    q,
    sortBy,
    hasComments
  );

  console.log(response);
}
[inline-code-end]