## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| id | String | Tak |  |

## Odpowiedź

Zwraca: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład delete_question_result'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete() -> Result<(), Error> {
    let tenant_id: String = "acme-corp-tenant".to_string();
    let id: String = "news/article-12345/question-67890".to_string();

    let params = DeleteQuestionResultParams {
        tenant_id,
        id,
    };

    let response: ApiEmptyResponse = delete_question_result(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---