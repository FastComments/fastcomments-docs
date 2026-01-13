## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| id | String | Tak |  |

## Odpowiedź

Zwraca: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Przykład

[inline-code-attrs-start title = 'delete_question_config Przykład'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteQuestionConfigParams = DeleteQuestionConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article-comments-config-2025".to_string(),
        force: Some(true),
    };
    let response: FlagCommentPublic200Response = delete_question_config(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---