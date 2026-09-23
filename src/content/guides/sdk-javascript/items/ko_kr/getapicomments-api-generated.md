## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| page | number | No |  |
| count | number | No |  |
| textSearch | string | No |  |
| byIPFromComment | string | No |  |
| filters | string | No |  |
| searchFilters | string | No |  |
| sorts | string | No |  |
| demo | boolean | No |  |
| sso | string | No |  |

## 응답

반환: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetCommentsResponse.ts)

## 예제

[inline-code-attrs-start title = 'getApiComments 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchComments(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const page: number = 2;
  const count: number = 20;
  const textSearch: string = "typescript";
  const byIPFromComment: string = "192.168.1.100";
  const filters: string = "spam,offensive";
  const searchFilters: string = "user:john";
  const sorts: string = "date_desc";
  const demo: boolean = true;
  const sso: string = "sso_token_abc";

  const response: ModerationAPIGetCommentsResponse = await getApiComments(
    tenantId,
    page,
    count,
    textSearch,
    byIPFromComment,
    filters,
    searchFilters,
    sorts,
    demo,
    sso
  );
}

fetchComments();
[inline-code-end]

---