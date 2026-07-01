## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_question_result_body | models::CreateQuestionResultBody | Yes |  |

## Відповідь

Повертає: [`CreateQuestionResultResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_question_result_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад create_question_result'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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