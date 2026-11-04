מידע משתמשים בכמות גדולה עבור טננט. בהינתן userIds, מחזיר מידע תצוגה מ-User / SSOUser.
משמש את ווידג'ט התגובות להעשרת משתמשים שהופיעו זה עתה באמצעות אירוע נוכחות.
ללא הקשר עמוד: הפרטיות נאכפת באופן אחיד (פרופילים פרטיים מוסתרים).

## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| ids | String | Yes |  |

## תשובה

מחזיר: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_users_info'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "user-1,user-2".to_string(),
};
let page: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]