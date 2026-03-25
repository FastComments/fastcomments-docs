## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| id | String | Evet |  |
| update_question_result_body | models::UpdateQuestionResultBody | Evet |  |

## Yanıt

Döndürür: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Örnek

[inline-code-attrs-start title = 'update_question_result Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_update_question_result() -> Result<(), Error> {
    let params: UpdateQuestionResultParams = UpdateQuestionResultParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article/2026/03/25/12345".to_string(),
        update_question_result_body: models::UpdateQuestionResultBody {
            question_id: Some("q-987".to_string()),
            result: Some(true),
            reviewer_id: Some("moderator-7".to_string()),
            notes: Some("Marked as resolved after editorial review".to_string()),
        },
    };
    let response: FlagCommentPublic200Response = update_question_result(&configuration, params).await?;
    println!("update result: {:?}", response);
    Ok(())
}
[inline-code-end]

---