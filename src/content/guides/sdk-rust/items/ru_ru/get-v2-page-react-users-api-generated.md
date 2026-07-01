## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| url_id | String | Да |  |
| id | String | Да |  |

## Response

Возвращает: `GetV2PageReactUsersResponse`

## Example

[inline-code-attrs-start title = 'get_v2_page_react_users Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---