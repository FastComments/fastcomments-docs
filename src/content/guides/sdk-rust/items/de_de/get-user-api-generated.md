## Parameter

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |

## Antwort

Gibt zurück: [`GetUser200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_200_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_user Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
pub async fn run_get_user_example() -> Result<(), Error> {
    let tenant: Option<String> = Some("acme-corp-tenant".to_string());
    let params: GetUserParams = GetUserParams {
        tenant_id: tenant.unwrap(),
        id: "user-9f8b3c".to_string(),
    };
    let user: GetUser200Response = get_user(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---