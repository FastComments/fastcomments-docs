אובייקט `AuditLog` מייצג אירוע מבוקר עבור שוכרים שיש להם גישה לתכונה זו.

המבנה עבור אובייקט AuditLog הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string;
    userId?: string;
    username?: string;
    resourceName: string;
    crudType: 'c' | 'r' | 'u' | 'd' | 'login';
    from: string;
    url?: string;
    ip?: string;
    when: string;
    description?: string;
    serverStartDate: string;
    objectDetails?: object;
}
[inline-code-end]

יומן הביקורת הוא בלתי ניתן לשינוי. גם לא ניתן לכתוב אליו ידנית. FastComments.com עשוי להחליט מתי לכתוב ליומן הביקורת. עם זאת, אתה יכול לקרוא ממנו דרך API זה.

אירועים ביומן הביקורת פגים לאחר שנתיים.
