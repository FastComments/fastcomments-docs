## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Response

Returns: [`DeletePageApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_page_api_response.rs)

## Example

[inline-code-attrs-start title = 'delete_page Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete_page() -> Result<DeletePageApiResponse, Error> {
    let params: DeletePageParams = DeletePageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article".to_string(),
    };

    let result: DeletePageApiResponse = delete_page(&configuration, params).await?;
    Ok(result)
}
[inline-code-end]
