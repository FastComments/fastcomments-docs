## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| textSearch | string | Ne |  |
| byIPFromComment | string | Ne |  |
| filters | string | Ne |  |
| searchFilters | string | Ne |  |
| afterId | string | Ne |  |
| demo | boolean | Ne |  |
| tenantId | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`GetApiIdsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetApiIdsResponse.ts)

## Primjer

[inline-code-attrs-start title = 'getApiIds Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---