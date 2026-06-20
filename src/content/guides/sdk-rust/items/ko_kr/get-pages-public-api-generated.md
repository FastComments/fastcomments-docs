테넌트의 페이지 목록을 나열합니다. FChat 데스크톱 클라이언트가 방 목록을 채우는 데 사용됩니다.
각 페이지에 대한 확인된 사용자 정의 구성에서 `enableFChat`이 true여야 합니다.
SSO가 필요한 페이지는 요청 사용자의 그룹 접근 권한을 기준으로 필터링됩니다.

## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| cursor | String | No |  |
| limit | i32 | No |  |
| q | String | No |  |
| sort_by | models::PagesSortBy | No |  |
| has_comments | bool | No |  |

## 응답

반환: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## 예제

[inline-code-attrs-start title = 'get_pages_public 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetPagesPublicParams = GetPagesPublicParams {
    tenant_id: String::from("acme-corp-tenant"),
    cursor: Some(String::from("cursor_eyJwZl9pZCI6IjEyMyJ9")),
    limit: Some(50),
    q: Some(String::from("tag:release status:published")),
    sort_by: Some(models::PagesSortBy::CreatedAt),
    has_comments: Some(true),
};
let response: GetPublicPagesResponse = get_pages_public(&configuration, params).await?;
[inline-code-end]

---