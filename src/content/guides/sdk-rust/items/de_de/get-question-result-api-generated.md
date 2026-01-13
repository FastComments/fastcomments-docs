## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |

## Antwort

Gibt zurück: [`GetQuestionResult200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_result_200_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_question_result Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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