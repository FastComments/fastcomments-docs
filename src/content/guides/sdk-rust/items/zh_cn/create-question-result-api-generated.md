## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| create_question_result_body | models::CreateQuestionResultBody | 是 |  |

## 响应

返回: [`CreateQuestionResultResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_question_result_response.rs)

## 示例

[inline-code-attrs-start title = 'create_question_result 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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