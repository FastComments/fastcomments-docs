List pages for a tenant. Used by the FChat desktop client to populate its room list.  
테넌트의 페이지 목록을 반환합니다. FChat 데스크톱 클라이언트가 방 목록을 채우는 데 사용됩니다.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
`enableFChat`이 각 페이지에 대해 해결된 맞춤 설정에서 true이어야 합니다.  
Pages that require SSO are filtered against the requesting user's group access.  
SSO가 필요한 페이지는 요청한 사용자의 그룹 접근 권한에 따라 필터링됩니다.

## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| cursor | String | No |  |
| limit | i32 | No |  |
| q | String | No |  |
| sort_by | models::PagesSortBy | No |  |
| has_comments | bool | No |  |

## 응답

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

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