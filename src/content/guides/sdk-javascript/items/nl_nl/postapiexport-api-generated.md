## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| textSearch | string | Nee |  |
| byIPFromComment | string | Nee |  |
| filters | string | Nee |  |
| searchFilters | string | Nee |  |
| sorts | string | Nee |  |
| sso | string | Nee |  |

## Respons

Geeft terug: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationExportResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'postApiExport Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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