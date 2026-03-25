## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| create_question_result_body | models::CreateQuestionResultBody | はい |  |

## 戻り値

戻り値: [`CreateQuestionResult200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_question_result_200_response.rs)

## 例

[inline-code-attrs-start title = 'create_question_result の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_create_question_result() -> Result<(), Error> {
    let params: CreateQuestionResultParams = CreateQuestionResultParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_question_result_body: models::CreateQuestionResultBody {
            question_id: "article-123-comment-rating".to_string(),
            user_id: Some("reader-456".to_string()),
            result: Some("helpful".to_string()),
            context: Some("news/article".to_string()),
            submitted_at: Some("2026-03-25T12:34:56Z".to_string()),
        },
    };

    let response: CreateQuestionResult200Response = create_question_result(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---