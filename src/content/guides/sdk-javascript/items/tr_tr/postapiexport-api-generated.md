## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| textSearch | string | Hayır |  |
| byIPFromComment | string | Hayır |  |
| filters | string | Hayır |  |
| searchFilters | string | Hayır |  |
| sorts | string | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationExportResponse.ts)

## Örnek

[inline-code-attrs-start title = 'postApiExport Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const textSearch: string = "suspicious link";
const byIPFromComment: string = "203.0.113.45";
const filters: string = "status:flagged,platform:web";
const searchFilters: string | undefined = undefined;
const sorts: string = "-createdAt";
const sso: string = "sso_token_3f9b8";

const exportResponse: ModerationExportResponse = await postApiExport(
  textSearch,
  byIPFromComment,
  filters,
  searchFilters,
  sorts,
  sso
);
[inline-code-end]

---