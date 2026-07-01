## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| user_id | String | Sí |  |
| create_ticket_body | models::CreateTicketBody | Sí |  |

## Respuesta

Devuelve: [`CreateTicketResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_ticket_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo create_ticket'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let body = models::CreateTicketBody {
        title: "Login Issue".to_string(),
        description: "User cannot log in after password reset".to_string(),
        priority: Some("high".to_string()),
    };
    let params = CreateTicketParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: "user-12345".to_string(),
        create_ticket_body: body,
    };
    let _response: CreateTicketResponse = create_ticket(&configuration, params).await?;
    Ok(())
}
[inline-code-end]