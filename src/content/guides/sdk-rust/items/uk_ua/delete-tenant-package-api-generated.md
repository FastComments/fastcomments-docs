## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| id | String | Так |  |

## Відповідь

Повертає: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад використання delete_tenant_package'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete_example() -> Result<(), Error> {
    let params: DeleteTenantPackageParams = DeleteTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "premium-comment-moderation".to_string(),
    };
    let response: ApiEmptyResponse = delete_tenant_package(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---