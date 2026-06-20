## Параметри

| Назва | Тип | Обов'язкове | Опис |
|------|------|----------|-------------|
| textSearch | string | Ні |  |
| byIPFromComment | string | Ні |  |
| filters | string | Ні |  |
| searchFilters | string | Ні |  |
| sorts | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationExportResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад postApiExport'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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