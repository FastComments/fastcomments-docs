אם ה‑val שלך כבר יודע מי המבקר, Secure SSO מעביר את הזהות לווידג'ט כך שהם לעולם לא יראו התחברות שנייה. אין צורך לבנות נקודות קצה ואין מה לקרוא בזמן ריצה: אתה מחשב שלושה ערכים בצד השרת ומעביר אותם בתצורת הווידג'ט.

Val Town מספק כניסה ללא צורך בתצורה עם `std/oauth`, כך שהמבקר יכול להתחבר עם חשבון Val Town שכבר יש לו. החלף זאת בכל מה שהאפליקציה שלך משתמשת בו; החלק של FastComments אינו משתנה.

## Build the payload on the server

הסוד של ה‑API חותם על המטען ולא צריך להגיע לקוד בדפדפן. התקן את ה‑SDK מ‑npm, אשר פועל על סביבת הריצה Deno של Val Town כפי שהיא:

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

`getPayload()` מחזיר `{ userDataJSONBase64, verificationHash, timestamp }`. שלושת הערכים האלה הם כל מה שמגיע לדפדפן. הסוד חותם עליהם ולאחר מכן נזרק, ולכן שום דבר בעמוד אינו מאפשר לקורא לזייף משתמש שונה.

## Pass it to the widget

[inline-code-attrs-start title = 'תצורת וידג\'ט עם SSO'; type='javascript' inline-code-attrs-end]
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

`oauthMiddleware` מוסיף עבורך `GET /auth/login`, `GET /auth/callback` ו‑`POST /auth/logout`. שים לב שהיציאה (logout) היא **POST**, בעוד שהווידג'ט מנווט ל‑`logoutURL` עם GET, ולכן הפנה את `logoutURL` לנתיב קטן משלך שמבצע את ה‑POST.

כאשר המבקר מנותק, העבר `sso` עם רק `loginURL`. הווידג'ט יציג אז תזכורת התחברות במקום תיבת תגובה אנונימית.

## Things that go wrong

`timestamp` הוא זמן אפוק (epoch) במילישניות, אסור שיהיה בעתיד, וחייב להיות לא יותר משני ימים ישנים. צור אותו בצד השרת באותו בקשה שמחשבת את החתימה. יצירתו בדפדפן היא הכשל הקלאסי: הערך שונה מהערך שחולץ ולכן כל תגובה נדחית.

לעולם אל תגדיר `isAdmin` או `isModerator` מספק הזהות. התחברות עם חשבון Val Town לא אומרת דבר על מי צריך למודרציה של האתר שלך.

ראה את [מדריך SSO](/guide-sso.html) לקבלת רשימת השדות המלאה, נושאים עם גישה קבוצתית, ותגים.