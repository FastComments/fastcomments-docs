## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| commentId | string | כן |  |
| banEmail | boolean | לא |  |
| banEmailDomain | boolean | לא |  |
| banIP | boolean | לא |  |
| deleteAllUsersComments | boolean | לא |  |
| bannedUntil | string | לא |  |
| isShadowBan | boolean | לא |  |
| updateId | string | לא |  |
| banReason | string | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-postBanUserFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_9f8a7b4e";
const bannedUntil: string = new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString();
const sso: string = "sso-user-7f3b2c";
const updateId: string = "upd_20260619_001";
const banReason: string = "Repeated harassment across multiple threads";
const result: BanUserFromCommentResult = await postBanUserFromComment(
  commentId,
  true,        // חסימת דואר אלקטרוני
  false,       // חסימת תחום דואר אלקטרוני
  true,        // חסימת כתובת IP
  true,        // מחיקת כל התגובות של המשתמש
  bannedUntil,
  false,       // חסימה סמויה
  updateId,
  banReason,
  sso
);
[inline-code-end]