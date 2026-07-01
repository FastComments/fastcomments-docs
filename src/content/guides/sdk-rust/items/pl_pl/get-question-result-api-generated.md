---
## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenant_id | String | Tak |  |
| id | String | Tak |  |

## Odpowiedź

Zwraca: [`GetQuestionResultResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_result_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład get_question_result'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_question_result(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetQuestionResultParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "question-12345".to_string(),
        locale: Some("en-US".to_string()),
    };
    let _response: GetQuestionResultResponse = get_question_result(config, params).await?;
    Ok(())
}
[inline-code-end]

---