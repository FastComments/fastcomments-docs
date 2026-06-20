## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| page | number | לא |  |
| count | number | לא |  |
| textSearch | string | לא |  |
| byIPFromComment | string | לא |  |
| filters | string | לא |  |
| searchFilters | string | לא |  |
| sorts | string | לא |  |
| demo | boolean | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetCommentsResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getApiComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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