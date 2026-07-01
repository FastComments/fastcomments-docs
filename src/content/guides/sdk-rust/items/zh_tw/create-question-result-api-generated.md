---
## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_question_result_body | models::CreateQuestionResultBody | Yes |  |

## 回應

返回：[`CreateQuestionResultResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_question_result_response.rs)

## 範例

[inline-code-attrs-start title = 'create_question_result 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---