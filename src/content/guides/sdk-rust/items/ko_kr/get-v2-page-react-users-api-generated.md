---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| url_id | String | 예 |  |
| id | String | 예 |  |

## 응답

반환: `GetV2PageReactUsersResponse`

## 예제

[inline-code-attrs-start title = 'get_v2_page_react_users 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_react_users(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params: GetV2PageReactUsersParams = GetV2PageReactUsersParams {
        tenant_id: String::from("acme-corp-tenant"),
        url_id: String::from("news/2026/space-flight-updates"),
        id: String::from("page-7a3f"),
        include_reaction_info: Some(true),
        limit: Some(100),
    };
    let response: GetV2PageReactUsersResponse = get_v2_page_react_users(configuration, params).await?;
    let _response = response;
    Ok(())
}
[inline-code-end]

---