---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## 응답

반환: [`GetPagesAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesAPIResponse.ts)

## 예제

[inline-code-attrs-start title = 'getPages 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

(async () => {
  const response: GetPagesAPIResponse = await getPages(tenantId);
  const firstPage: APIPage | undefined = response.pages?.[0];
  console.log(firstPage?.title);
})();
[inline-code-end]

---