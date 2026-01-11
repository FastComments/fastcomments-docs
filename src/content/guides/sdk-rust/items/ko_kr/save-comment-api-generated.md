## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| create_comment_params | models::CreateCommentParams | 예 |  |
| is_live | bool | 아니오 |  |
| do_spam_check | bool | 아니오 |  |
| send_emails | bool | 아니오 |  |
| populate_notifications | bool | 아니오 |  |

## 응답

반환: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/save_comment_200_response.rs)

---