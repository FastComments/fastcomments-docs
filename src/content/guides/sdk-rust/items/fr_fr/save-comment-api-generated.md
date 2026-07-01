## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|--------------|-------------|
| tenant_id | String | Oui |  |
| create_comment_params | models::CreateCommentParams | Oui |  |
| is_live | bool | Non |  |
| do_spam_check | bool | Non |  |
| send_emails | bool | Non |  |
| populate_notifications | bool | Non |  |

## Réponse

Retourne : [`ApiSaveCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_save_comment_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de save_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = SaveCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_comment_params: models::CreateCommentParams {
            body: "Great insights on the latest tech trends.".to_string(),
            user_id: "user-789".to_string(),
            ..Default::default()
        },
        is_live: Some(true),
        do_spam_check: Some(true),
        send_emails: Some(false),
        populate_notifications: Some(true),
    };
    let _response = save_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]