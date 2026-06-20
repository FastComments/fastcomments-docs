מידע מאסיבי על משתמשים עבור tenant. בהינתן userIds, החזר מידע תצוגה מתוך User / SSOUser.
משמש על ידי רכיב התגובות כדי להעשיר משתמשים שהופיעו זה עתה באמצעות אירוע נוכחות.
אין הקשר של עמוד: פרטיות נאכפת באופן אחיד (פרופילים פרטיים מוסתרים).

## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| ids | String | כן |  |

## תגובה

מחזיר: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_users_info'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetUsersInfoParams = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "alice@example.com,bob@example.com,carol@example.com".to_string(),
    page_size: Some(100),
};
let users_response: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]

---