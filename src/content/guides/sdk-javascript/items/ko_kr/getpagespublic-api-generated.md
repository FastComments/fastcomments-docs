테넌트의 페이지를 나열합니다. FChat 데스크톱 클라이언트가 룸 목록을 채우는 데 사용됩니다.
Requires `enableFChat` to be true on the resolved custom config for each page.
SSO가 필요한 페이지는 요청한 사용자의 그룹 접근 권한에 따라 필터링됩니다.

## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| cursor | string | 아니오 |  |
| limit | number | 아니오 |  |
| q | string | 아니오 |  |
| sortBy | PagesSortBy | 아니오 |  |
| hasComments | boolean | 아니오 |  |

## 응답

반환: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## 예제

[inline-code-attrs-start title = 'getPagesPublic 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f9b2c";
const cursor: string = "cursor_0001a2b3";
const limit: number = 25;
const q: string = "product page";
const hasComments: boolean = true;
const response: GetPublicPagesResponse = await getPagesPublic(tenantId, cursor, limit, q, undefined, hasComments);
[inline-code-end]

---