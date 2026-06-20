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
let params: CreateQuestionResultParams = CreateQuestionResultParams {
    tenant_id: String::from("acme-corp-tenant"),
    create_question_result_body: models::CreateQuestionResultBody {
        question_id: String::from("news/article/1234"),
        user_id: Some(String::from("reader-9876")),
        answer: String::from("B"),
        correct: Some(false),
        score: Some(0.0),
    },
};
let response: CreateQuestionResultResponse = create_question_result(&configuration, params).await?;
[inline-code-end]

---