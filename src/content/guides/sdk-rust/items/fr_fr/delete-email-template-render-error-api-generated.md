## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| id | String | Oui |  |
| error_id | String | Oui |  |

## Réponse

Renvoie : [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de delete_email_template_render_error'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn delete_email_template_render_error_example() -> Result<FlagCommentPublic200Response, Error> {
    let params: DeleteEmailTemplateRenderErrorParams = DeleteEmailTemplateRenderErrorParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "marketing/newsletter/welcome-email".to_string(),
        error_id: "render-err-2026-01-12-01".to_string(),
    };
    let response: FlagCommentPublic200Response = delete_email_template_render_error(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---