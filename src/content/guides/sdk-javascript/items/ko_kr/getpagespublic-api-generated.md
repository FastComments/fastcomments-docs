---
테넌트의 페이지를 나열합니다. FChat 데스크톱 클라이언트가 방 목록을 채우는 데 사용됩니다.  
각 페이지에 대해 해결된 커스텀 구성에서 `enableFChat`이 true이어야 합니다.  
SSO가 필요한 페이지는 요청한 사용자의 그룹 액세스를 기준으로 필터링됩니다.

## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | 예 |  |
| cursor | string | 아니오 |  |
| limit | number | 아니오 |  |
| q | string | 아니오 |  |
| sortBy | PagesSortBy | 아니오 |  |
| hasComments | boolean | 아니오 |  |

## 응답

반환: [`GetPagesPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublicResponse.ts)

## 예시

[inline-code-attrs-start title = 'getPagesPublic 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPages() {
  const tenantId: string = "tenant_12345";
  const cursor: string = "nextPageToken";
  const limit: number = 20;
  const q: string = "blog";
  const sortBy: PagesSortBy = "createdAt";
  const hasComments: boolean = true;

  const response: GetPagesPublicResponse = await getPagesPublic(
    tenantId,
    cursor,
    limit,
    q,
    sortBy,
    hasComments
  );

  console.log(response);
}
[inline-code-end]