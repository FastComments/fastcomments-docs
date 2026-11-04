테넌트의 페이지 목록을 조회합니다. FChat 데스크톱 클라이언트에서 방 목록을 채우는 데 사용됩니다.
각 페이지의 확인된 커스텀 설정에서 `enableFChat`이 true여야 합니다.
SSO가 필요한 페이지는 요청 사용자의 그룹 접근 권한을 기준으로 필터링됩니다.

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| cursor | String | 아니요 |  |
| limit | i32 | 아니요 |  |
| q | String | 아니요 |  |
| sort_by | models::PagesSortBy | 아니요 |  |
| has_comments | bool | 아니요 |  |

## 응답

반환값: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## 예제

[inline-code-attrs-start title = 'get_pages_public 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetPagesPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        cursor: Some("page_20".to_string()),
        limit: Some(50),
        q: Some("news/article".to_string()),
        sort_by: Some(models::PagesSortBy::CreatedDesc),
        has_comments: Some(true),
    };
    let _response = get_pages_public(configuration, params).await?;
    Ok(())
}
[inline-code-end]