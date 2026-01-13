## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| after_id | String | 아니오 |  |
| after_created_at | i64 | 아니오 |  |
| unread_only | bool | 아니오 |  |
| dm_only | bool | 아니오 |  |
| no_dm | bool | 아니오 |  |
| sso | String | 아니오 |  |

## 응답

반환: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/reset_user_notifications_200_response.rs)

---