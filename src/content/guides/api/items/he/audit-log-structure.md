---
An `AuditLog` הוא אובייקט שמייצג אירוע מבוקר עבור שוכרים שיש להם גישה לתכונה זו.

המבנה של אובייקט AuditLog הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string;
    /** מי ביצע את האירוע. **/
    userId?: string;
    username?: string;
    resourceName: string;
    crudType: 'c' | 'r' | 'u' | 'd' | 'login';
    from: string;
    url?: string;
    ip?: string;
    /** הדפדפן שביצע את האירוע, כאשר הוא הגיע מדפדפן. **/
    ua?: string;
    /** האש של הסשן שממנו הגיע האירוע, לצורך קישור בין פעולות של אדם אחד. לעולם לא את הסשן עצמו. **/
    sIdHashed?: string;
    when: string;
    description?: string;
    serverStartDate: string;
    /** המזהה של האובייקט שעליו בוצע האירוע, בניגוד למי שביצע אותו. **/
    targetId?: string;
    /** תווית קריאה לבן אדם עבור האובייקט, לדוגמה "jsmith (jsmith@example.com)". **/
    targetLabel?: string;
    objectDetails?: object;
}
[inline-code-end]

`targetId` ו-`targetLabel` מתארים על מה בוצע האירוע; `userId` ו-`username` מתארים מי ביצע אותו. עבור עדכונים, `objectDetails.changes` מכיל מפת `{field: {from, to}}` של מה שבאמת השתנה.

יומן הבדיקה הוא בלתי ניתן לשינוי. הוא גם אינו ניתן לכתיבה ידנית. FastComments.com יכולה בלבד להחליט מתי לכתוב ליומן הבדיקה. עם זאת, ניתן לקרוא ממנו דרך ה-API הזה.

אירועים ביומן הבדיקה פוגים לאחר שנתיים.
---