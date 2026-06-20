## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| textSearch | string | 아니요 |  |
| byIPFromComment | string | 아니요 |  |
| filters | string | 아니요 |  |
| searchFilters | string | 아니요 |  |
| sorts | string | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationExportResponse.ts)

## 예제

[inline-code-attrs-start title = 'postApiExport 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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