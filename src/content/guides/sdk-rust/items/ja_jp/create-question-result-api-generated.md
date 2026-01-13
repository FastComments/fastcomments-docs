## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| create_question_result_body | models::CreateQuestionResultBody | はい |  |

## レスポンス

返却値: [`CreateQuestionResult200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_question_result_200_response.rs)

## 例

[inline-code-attrs-start title = 'create_question_result の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: CreateQuestionResultParams = CreateQuestionResultParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_question_result_body: models::CreateQuestionResultBody {
            question_id: "feedback-article-2026".to_string(),
            comment_id: Some("cmt-2026-001".to_string()),
            user_id: Some("reader-007".to_string()),
            answer: "yes".to_string(),
            score: Some(4),
            metadata: Some(std::collections::HashMap::from([(
                "path".to_string(),
                "news/politics/2026-election".to_string(),
            )])),
            anonymous: Some(false),
            submitted_at: Some("2026-01-12T09:15:00Z".to_string()),
        },
    };

    let created: CreateQuestionResult200Response = create_question_result(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---