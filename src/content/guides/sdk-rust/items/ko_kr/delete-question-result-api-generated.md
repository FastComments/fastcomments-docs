---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |

## 응답

반환: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 예제

[inline-code-attrs-start title = 'delete_question_result 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete() -> Result<(), Error> {
    let tenant_id: String = "acme-corp-tenant".to_string();
    let id: String = "news/article-12345/question-67890".to_string();

    let params = DeleteQuestionResultParams {
        tenant_id,
        id,
    };

    let response: ApiEmptyResponse = delete_question_result(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---