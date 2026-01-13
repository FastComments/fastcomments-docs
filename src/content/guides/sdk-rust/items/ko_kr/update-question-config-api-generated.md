## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |
| update_question_config_body | models::UpdateQuestionConfigBody | 예 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 예제

[inline-code-attrs-start title = 'update_question_config 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_update() -> Result<(), Error> {
    let params: UpdateQuestionConfigParams = UpdateQuestionConfigParams {
        tenant_id: String::from("acme-corp-tenant"),
        id: String::from("news/article-2026-01-12"),
        update_question_config_body: models::UpdateQuestionConfigBody {
            question_text: String::from("Did you find this reporting accurate?"),
            required: Some(true),
            rendering_type: Some(models::QuestionRenderingType::Inline),
            options: Some(vec![
                models::QuestionConfigCustomOptionsInner { id: String::from("opt-yes"), label: String::from("Yes"), value: String::from("yes") },
                models::QuestionConfigCustomOptionsInner { id: String::from("opt-no"), label: String::from("No"), value: String::from("no") },
            ]),
            when_save: Some(models::QuestionWhenSave::OnSubmit),
        },
    };
    let response: FlagCommentPublic200Response = update_question_config(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---