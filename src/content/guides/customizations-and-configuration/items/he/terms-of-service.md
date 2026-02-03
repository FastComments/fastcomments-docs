FastComments מאפשרת לך לדרוש ממגיבים בפעם הראשונה לקבל את תנאי השירות שלך לפני שליחת תגובה.

When enabled:
- **משתמשים אנונימיים** יראו תיבת סימון לאישור תנאי השירות בכל פעם שהם מגיבים
- **משתמשים מאומתים** יראו את תיבת הסימון רק בתגובה הראשונה שלהם, או כשאתה מעדכן את תנאי השירות

### Enabling Terms of Service

נווט לעמוד התאמת הווידג'ט והפעל את תיבת הסימון "דרוש אישור תנאי השירות":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-enabled'; title='Enable Terms of Service Checkbox' app-screenshot-end]

### Customizing the TOS Text

בברירת המחדל, תיבת הסימון מציגה "אני מסכים לתנאי השירות ולמדיניות הפרטיות" עם קישורים לשני המסמכים. ניתן להתאים טקסט זה לפי לוקאל במידת הצורך:

1. בחר "התאם טקסט לפי לוקאל"
2. בחר את הלוקאל מהרשימה וזן את הטקסט המותאם שלך

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-text-mode'; title='Customize TOS Text' app-screenshot-end]

### Updating Your Terms of Service

כאשר אתה מעדכן את תנאי השירות שלך, קבע את תאריך ה-"Last Updated". משתמשים שאישרו את תנאי השירות לפני תאריך זה יידרשו לאשר שוב:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-last-updated'; title='TOS Last Updated Date' app-screenshot-end]

### How It Works

- חותמת הזמן של אישור תנאי השירות נשמרת לפי משתמש ולפי תגובה
- כאשר משתמש מאשר את תנאי השירות, התאריך נרשם בפרופיל המשתמש שלו (ברמת ה-tenant)
- אם אתה קובע תאריך "Last Updated" שהוא לאחר תאריך אישור המשתמש, הם יצטרכו לאשר שוב
- עבור משתמשים אנונימיים שלא ניתן לעקוב אחריהם, תיבת הסימון תופיע בכל שליחת תגובה