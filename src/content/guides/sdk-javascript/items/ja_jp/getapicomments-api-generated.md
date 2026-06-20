## パラメータ

| Name | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| page | number | いいえ |  |
| count | number | いいえ |  |
| textSearch | string | いいえ |  |
| byIPFromComment | string | いいえ |  |
| filters | string | いいえ |  |
| searchFilters | string | いいえ |  |
| sorts | string | いいえ |  |
| demo | boolean | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetCommentsResponse.ts)

## 例

[inline-code-attrs-start title = 'getApiComments の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const page: number = 2;
const count: number = 25;
const textSearch: string = 'comments failing to load after posting';
const filters: string = 'status:pending,moderation:required';
const sorts: string = 'createdAt:desc';
const demo: boolean = false;
const sso: string = 'sso-usr-7f3b2a';

const response: ModerationAPIGetCommentsResponse = await getApiComments(page, count, textSearch, undefined, filters, undefined, sorts, demo, sso);
[inline-code-end]

---