## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| create_comment_params | models::CreateCommentParams | Sim |  |
| is_live | bool | Não |  |
| do_spam_check | bool | Não |  |
| send_emails | bool | Não |  |
| populate_notifications | bool | Não |  |

## Resposta

Retorna: [`ApiSaveCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_save_comment_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo save_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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