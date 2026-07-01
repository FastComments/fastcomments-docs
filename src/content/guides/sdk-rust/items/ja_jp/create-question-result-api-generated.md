## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| create_question_result_body | models::CreateQuestionResultBody | はい |  |

## レスポンス

戻り値: [`CreateQuestionResultResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_question_result_response.rs)

## 例

[inline-code-attrs-start title = 'create_question_result の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let mut metadata = std::collections::HashMap::new();
metadata.insert("source".to_string(), "web".to_string());

let body = models::CreateQuestionResultBody {
    question_id: "q-987".to_string(),
    user_id: "user-42".to_string(),
    answer: "Positive".to_string(),
    metadata: Some(metadata),
};

let params = CreateQuestionResultParams {
    tenant_id: "acme-corp-tenant".to_string(),
    create_question_result_body: body,
};

let response = create_question_result(&configuration, params).await?;
[inline-code-end]