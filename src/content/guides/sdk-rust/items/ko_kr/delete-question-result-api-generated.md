## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 예제

[inline-code-attrs-start title = 'delete_question_result 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete_question_result() -> Result<(), Error> {
    let params: DeleteQuestionResultParams = DeleteQuestionResultParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "qres-news/article-2026-03-25-9a8b7c".to_string(),
    };
    let response: FlagCommentPublic200Response = delete_question_result(&configuration, params).await?;
    let _response = response;
    Ok(())
}
[inline-code-end]

---