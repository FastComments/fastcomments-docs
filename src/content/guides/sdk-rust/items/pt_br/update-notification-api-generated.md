## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenant_id | String | Sim |  |
| id | String | Sim |  |
| update_notification_body | models::UpdateNotificationBody | Sim |  |
| user_id | String | Não |  |

## Resposta

Retorna: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de update_notification'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: UpdateNotificationParams = UpdateNotificationParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "notif-12345".to_string(),
    update_notification_body: models::UpdateNotificationBody {
        name: "Article Comments Webhook".to_string(),
        url: "https://hooks.acme-corp.com/article-comments".to_string(),
        enabled: true,
        events: vec!["comment.created".to_string(), "comment.flagged".to_string()],
    },
    user_id: Some("moderator-42".to_string()),
};

let response: FlagCommentPublic200Response = update_notification(&configuration, params).await?;
[inline-code-end]

---