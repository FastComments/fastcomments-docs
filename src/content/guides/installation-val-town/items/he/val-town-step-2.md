`tenantId: "demo"` הוא ארגז חול ציבורי משותף. הוא פועל ללא הרשמה, ולכן הדוגמאות משתמשות בו, אך כל משתמש אחר של FastComments כותב באותו השרשורים וכל אחד יכול למנהלם. יש להחליף לפני שמפרסמים משהו שחשוב לכם.

מזהה השוכר שלכם נמצא ב[דף סוד ה-API](https://fastcomments.com/auth/my-account/api-secret).

מזהה שוכר הוא ציבורי ושייך לקוד בדפדפן. סוד API אינו ציבורי, ואין צורך באחד בדף זה.

## קראו זאת ממשתנה סביבתי

ערכי Val Town הם ציבוריים בתוכנית החינמית, ולכן המקור שלהם קריא לכל. שמרו כל מידע רגיש במשתני סביבת, וקראו אותם עם `Deno.env.get`:

[inline-code-attrs-start title = 'config.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
export const TENANT_ID = Deno.env.get("FASTCOMMENTS_TENANT_ID") ?? "demo";

// Only accounts created on eu.fastcomments.com set this, to "eu".
export const REGION = Deno.env.get("FASTCOMMENTS_REGION") ?? "";

export const CDN = REGION === "eu"
  ? "https://cdn-eu.fastcomments.com"
  : "https://cdn.fastcomments.com";
[inline-code-end]

זה חשוב יותר מהרגיל ב-Val Town מסיבה שנייה: **שינוי ערך של val מעתיק מפתחות של משתני סביבת, אך לא את ערכיהם.** סוד שמור במשתנה סביבת אינו נושא את ה-val שלכם לחשבון של מישהו אחר. סוד שנכתב בקובץ כן נושא.

חזרה ל-`"demo"` משאירה את ה-val פעיל עבור כל מי שמשנה אותו לפני שהוא מגדיר שוכר משלו.

## חשבונות EU

חשבון, הנתונים שלו והמפתחות שלו ממוקמים באיזור אחד. אם החשבון שלכם נוצר ב-`eu.fastcomments.com`, כל תצורת וידג'ט גם צריכה `region: "eu"`, והסקריפטים נטענים מ-`cdn-eu.fastcomments.com`. אחרת, השאירו את שניהם כפי שהם.