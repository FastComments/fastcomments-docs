## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| textSearch | string | Nein |  |
| byIPFromComment | string | Nein |  |
| filters | string | Nein |  |
| searchFilters | string | Nein |  |
| afterId | string | Nein |  |
| demo | boolean | Nein |  |
| tenantId | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Returns: [`GetApiIdsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetApiIdsResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getApiIds Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const textSearch: string = "urgent feedback";
const byIPFromComment: string = "203.0.113.42";
const filters: string = "status:approved";
const afterId: string = "comment-789";
const demo: boolean = true;
const tenantId: string = "tenant-001";

const apiIds: GetApiIdsResponse = await getApiIds({
  textSearch,
  byIPFromComment,
  filters,
  afterId,
  demo,
  tenantId,
});
[inline-code-end]