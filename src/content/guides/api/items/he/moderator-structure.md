אובייקט `Moderator` מייצג קונפיגורציה עבור מנהל תוכן.

ישנם שלושה סוגים של מנהלי תוכן:

1. משתמשי מנהלים שיש להם את הדגל `isCommentModeratorAdmin`.
2. משתמשי SSO עם הדגל `isCommentModeratorAdmin`.
3. מגיבים רגילים, או משתמשי FastComments.com, שהוזמנו כמנהלי תוכן.

מבנה `Moderator` משמש לייצג את מצב ניהול התוכן של מקרה שימוש `3`.

אם אתה רוצה להזמין משתמש להיות מנהל תוכן, דרך ה-API, השתמש ב-API של `Moderator` על ידי יצירת `Moderator` ו`הזמנתם`.

אם למשתמש אין חשבון FastComments.com, אימייל ההזמנה יעזור לו להתחיל. אם כבר יש לו חשבון, הוא
יקבל גישת ניהול תוכן לשוכר שלך ומאפיין `userId` של אובייקט ה-`Moderator` יתעדכן להצביע על המשתמש שלו. לא תהיה לך גישת API
למשתמש שלהם, כי במקרה זה הוא שייך להם ומנוהל על ידי FastComments.com.

אם אתה דורש ניהול מלא של חשבון המשתמש, אנחנו ממליצים להשתמש ב-SSO, או להוסיף אותם כ-[משתמש שוכר](https://fastcomments.com/auth/my-account/users) ו
לאחר מכן להוסיף אובייקט `Moderator` למעקב אחר הסטטיסטיקות שלהם.

ניתן להשתמש במבנה `Moderator` כמנגנון מעקב סטטיסטיקות עבור מקרי שימוש `1` ו-`2`. לאחר יצירת המשתמש, הוסף אובייקט `Moderator`
עם `userId` שלו מוגדר והסטטיסטיקות שלו יעקבו ב-[דף מנהלי התגובות](https://fastcomments.com/auth/my-account/moderate-comments/moderators).

המבנה עבור אובייקט `Moderator` הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה מנהל תוכן'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Moderator {
    name: string
    email: string
    tenantId: string
    userId?: string|null
    acceptedInvite?: boolean
    markReviewedCount?: number
    deletedCount?: number
    markedSpamCount?: number
    approvedCount?: number
    editedCount?: number
    bannedCount?: number
    createdAt: string
    moderationGroupIds?: string[]|null
}
[inline-code-end]
