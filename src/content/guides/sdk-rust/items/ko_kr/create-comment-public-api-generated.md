## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| url_id | String | 예 |  |
| broadcast_id | String | 예 |  |
| comment_data | models::CommentData | 예 |  |
| session_id | String | 아니오 |  |
| sso | String | 아니오 |  |

## 응답

반환: [`CreateCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_comment_public_200_response.rs)

---