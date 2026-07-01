## Parameters

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| url_id | String | 예 |  |
| id | String | 예 |  |

## Response

반환값: `GetV2PageReactUsersResponse`

## Example

[inline-code-attrs-start title = 'get_v2_page_react_users 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetV2PageReactUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        id: "react567".to_string(),
        page: Some(1),
        per_page: Some(50),
        ..Default::default()
    };
    let _response = get_v2_page_react_users(configuration, params).await?;
    Ok(())
}
[inline-code-end]