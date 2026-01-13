## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| id | String | Da |  |

## Odgovor

Vraća: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Primjer

[inline-code-attrs-start title = 'Primjer delete_question_result'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete() -> Result<(), Error> {
    let params: DeleteQuestionResultParams = DeleteQuestionResultParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article/12345/question/67890".to_string(),
        dry_run: Some(false),
        request_id: Some("req-20260112-7a3b".to_string()),
    };
    let response: FlagCommentPublic200Response = delete_question_result(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---