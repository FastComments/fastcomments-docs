אובייקט `Subscription` מייצג מנוי עבור משתמש.

אובייקטי `Subscription` נוצרים כאשר משתמש לוחץ על פעמון ההתראות בווידג'ט התגובות ולוחץ על "הירשם לעמוד זה".

ניתן גם ליצור מנויים דרך ה-API.

קיום אובייקט `Subscription` גורם ליצירת אובייקטי `Notification`, ושליחת אימיילים, כאשר תגובות חדשות נשארות בשורש העמוד המשויך
שה-`Subscription` הוא עבורו. שליחת אימיילים תלויה בסוג המשתמש. למשתמשים רגילים זה תלוי ב-`optedInNotifications`. למשתמשי SSO זה תלוי ב-`optedInSubscriptionNotifications`. שים לב שלחלק מהאפליקציות אולי אין את הרעיון של עמוד נגיש לאינטרנט, במקרה זה פשוט הגדר את `urlId` ל-
מזהה הפריט שאתה נרשם אליו (אותו ערך עבור `urlId` שהיית מעביר לווידג'ט התגובות).

המבנה עבור אובייקט `Subscription` הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה מנוי'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** With SSO, the user id is in the format `<tenant id>:<user id>`. **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // date string
}
[inline-code-end]
