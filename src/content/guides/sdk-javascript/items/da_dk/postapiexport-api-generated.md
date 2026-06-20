## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| textSearch | string | Nej |  |
| byIPFromComment | string | Nej |  |
| filters | string | Nej |  |
| searchFilters | string | Nej |  |
| sorts | string | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationExportResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på postApiExport'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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