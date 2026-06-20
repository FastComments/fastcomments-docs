## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| create_user_badge_params | models::CreateUserBadgeParams | Sí |  |

## Respuesta

Devuelve: [`ApiCreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_create_user_badge_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de create_user_badge'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: CreateUserBadgeParams = CreateUserBadgeParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_user_badge_params: models::CreateUserBadgeParams {
            user_id: "user-7890".to_string(),
            badge_key: "top-commenter".to_string(),
            title: "Top Commenter".to_string(),
            description: Some("Consistently provided insightful comments".to_string()),
            image_url: Some("https://assets.news.example.com/badges/top-commenter.png".to_string()),
            is_visible: Some(true),
            expires_at: None,
        },
    };

    let response: ApiCreateUserBadgeResponse = create_user_badge(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---