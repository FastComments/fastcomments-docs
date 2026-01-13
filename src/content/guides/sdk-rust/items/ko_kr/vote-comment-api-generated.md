## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| comment_id | String | 예 |  |
| url_id | String | 예 |  |
| broadcast_id | String | 예 |  |
| vote_body_params | models::VoteBodyParams | 예 |  |
| session_id | String | 아니오 |  |
| sso | String | 아니오 |  |

## 응답

반환: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)

---