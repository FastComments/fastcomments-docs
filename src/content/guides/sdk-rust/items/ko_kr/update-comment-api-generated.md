## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |
| body | models::PickApiCommentPeriodUpdatableCommentFields | 예 |  |
| context_user_id | String | 아니오 |  |
| do_spam_check | bool | 아니오 |  |
| is_live | bool | 아니오 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

---