אם ה-val שלך כבר יודע מי המבקר, Secure SSO מעביר את הזהות הזו לווידג'ט כך שהמבקר לעולם לא יראה התחברות שנייה. אין נקודות קצה לבנות ואין דבר לקרוא בזמן ריצה: אתה מחשב שלושה ערכים בצד השרת ומעביר אותם בהגדרות הווידג'ט.

Val Town מספקת התחברות ללא הגדרות באמצעות `std/oauth`, כך שהמבקר יכול להתחבר עם חשבון ה-Val Town שכבר יש לו. החלף את זה בכל מה שהאפליקציה שלך משתמשת בו; החלק של FastComments אינו משתנה.

## בניית ה-payload בשרת

סוד ה-API חותם את ה-payload ואסור שיגיע לקוד בדפדפן. התקן את ה-SDK מ-npm, שעובד בסביבת ה-Deno של Val Town כמו שהוא:

[inline-code-attrs-start title = 'sso.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { SecureSSOPayloadBuilder } from "npm:fastcomments-sdk/server";

export function buildSSOPayload(user) {
  // id must be stable for the same person, or they get a new comment identity on every login.
  const id = `vt-${user.id}`;

  return new SecureSSOPayloadBuilder(Deno.env.get("FASTCOMMENTS_API_SECRET"), {
    id,
    // email is required and must be unique.
    email: user.email ?? `${id}@users.noreply.val.town`,
    // username is required and cannot be an email.
    username: user.username ?? id,
    displayName: user.username ?? undefined,
    avatar: user.links.profileImageUrl ?? undefined,
  }).getPayload();
}
[inline-code-end]

`getPayload()` מחזיר `{ userDataJSONBase64, verificationHash, timestamp }`. שלושת הערכים האלה הם כל מה שמגיע לדפדפן. הסוד חותם עליהם ואז נזרק, כך ששום דבר בדף לא מאפשר לקורא לזייף משתמש אחר.

## העברת זה לווידג'ט

[inline-code-attrs-start title = 'הגדרות ווידג\'ט עם SSO'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { getOAuthUserData, oauthMiddleware } from "https://esm.town/v/std/oauth/middleware.ts";

app.get("/", async (c) => {
  const session = await getOAuthUserData(c.req.raw);
  const user = session?.user;

  const config = {
    tenantId: TENANT_ID,
    urlId: "my-thread",
    ...(user
      ? { sso: { ...buildSSOPayload(user), logoutURL: "/logout" } }
      : { sso: { loginURL: "/auth/login" } }),
  };

  // ...render the widget with this config
});

export default oauthMiddleware(app.fetch);
[inline-code-end]

`oauthMiddleware` מוסיף עבורך את `GET /auth/login`, `GET /auth/callback` ו-`POST /auth/logout`. שים לב שההתנתקות היא **POST**, בעוד שהווידג'ט מנווט אל `logoutURL` עם GET, ולכן כוון את `logoutURL` לנתיב קטן משלך ששולח את ה-POST.

כאשר המבקר מנותק, העבר `sso` עם `loginURL` בלבד. הווידג'ט יציג אז בקשת התחברות במקום תיבת תגובות אנונימית.

## דברים שמשתבשים

`timestamp` הוא ביחידות של **מילישניות** מתחילת העידן, אסור שיהיה בעתיד, ואסור שיהיה ישן מיומיים. צור אותו בשרת באותה בקשה שבה מחשבים את ה-hash. יצירתו בדפדפן היא הכשל הקלאסי: הערך שונה מזה שעליו חושב ה-hash וכל תגובה נדחית.

לעולם אל תגדיר `isAdmin` או `isModerator` מספק הזהויות. התחברות עם חשבון Val Town אינה אומרת דבר על מי צריך לנהל את האתר שלך.

ראה את [מדריך ה-SSO](/guide-customizations-and-configuration.html#sso-secure) לרשימת השדות המלאה, שרשורים מוגבלי קבוצות ותגים.