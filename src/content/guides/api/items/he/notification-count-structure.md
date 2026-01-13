אובייקט `NotificationCount` מייצג את ספירת ההתראות שלא נקראו ומטא-נתונים עבור משתמש.

אם אין התראות שלא נקראו, לא יהיה `NotificationCount` עבור המשתמש.

אובייקטי `NotificationCount` נוצרים אוטומטית ולא ניתן ליצור אותם דרך ה-API. הם גם פגים לאחר שנה אחת.

אתה יכול לנקות את ספירת ההתראות שלא נקראו של משתמש על ידי מחיקת ה-`NotificationCount` שלו.

המבנה עבור אובייקט `NotificationCount` הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // user id
    count: number
    createdAt: string // date string
    expireAt: string // date string
}
[inline-code-end]
