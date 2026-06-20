---
## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| url_id | String | 예 |  |
| id | String | 예 |  |
| title | String | 아니오 |  |

## Response

Returns: `CreateV1PageReact`

## Example

[inline-code-attrs-start title = 'create_v2_page_react 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_create_react() -> Result<CreateV1PageReact, Error> {
    let params: CreateV2PageReactParams = CreateV2PageReactParams {
        tenant_id: String::from("acme-corp-tenant"),
        url_id: String::from("news/2026/product-launch"),
        id: String::from("react-like"),
        title: Some(String::from("Product Launch Coverage")),
    };
    let response: CreateV1PageReact = create_v2_page_react(&config, params).await?;
    Ok(response)
}
[inline-code-end]

---