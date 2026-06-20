## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| create_question_result_body | models::CreateQuestionResultBody | כן |  |

## תגובה

מחזיר: [`CreateQuestionResultResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_question_result_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-create_question_result'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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