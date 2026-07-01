## Parameters

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Da |  |
| create_question_result_body | models::CreateQuestionResultBody | Da |  |

## Response

Vraća: [`CreateQuestionResultResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_question_result_response.rs)

## Primjer

[inline-code-attrs-start title = 'create_question_result Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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