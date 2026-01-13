## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Odpowiedź

Zwraca: [`GetQuestionResult200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_result_200_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład get_question_result'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<GetQuestionResult200Response, Error> {
    let include_metadata: Option<bool> = Some(true);
    let params: GetQuestionResultParams = GetQuestionResultParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article/2026/12345".to_string(),
    };
    let response: GetQuestionResult200Response = get_question_result(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---