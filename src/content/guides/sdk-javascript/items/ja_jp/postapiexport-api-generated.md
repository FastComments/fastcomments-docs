## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| textSearch | string | いいえ |  |
| byIPFromComment | string | いいえ |  |
| filters | string | いいえ |  |
| searchFilters | string | いいえ |  |
| sorts | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

返却値: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationExportResponse.ts)

## 例

[inline-code-attrs-start title = 'postApiExport の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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