---
עקבו אחרי אותם הצעדים עבור `localhost` כפי שהייתם עושים בייצור. ודאו שיש לכם תחומי ייצור והגדרות סודות API.

ראשית, נווטו אל [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). ניתן לגשת לכך דרך Manage Data -> Webhooks.

הדף מציג את כל הווב-הוקים בחשבון שלכם:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='דף ניהול ווב-הוקים שמציג כל ווב-הוק עם ה-URL שלו, אירוע, תחום, שיטה, סטטוס וספירת אירועים בתור'; title='רשימת ווב-הוקים'; cacheBuster = 'v4' app-screenshot-end]

לחצו על **New Webhook** כדי להוסיף אחד. לכל ווב-הוק יש URL, אירוע תגובה אחד (נוצר, עודכן או נמחק), תחום, ושיטת HTTP:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks/new'; selector = '.content'; alt='טופס ווב-הוק חדש עם שדות URL, אירוע, תחום ושיטת HTTP בנוסף לכפתור שליחת מטען בדיקה'; title='ווב-הוק חדש'; cacheBuster = 'v4' app-screenshot-end]

כל ווב-הוק נשלח באופן עצמאי. ניתן לשלוח את אותו אירוע למספר נקודות קצה, וווב-הוק המוגדר ל**All Domains** מקבל תגובות מכל תחום גם כאשר קיים ווב-הוק ספציפי לתחום עבור אותו אירוע. לא ניתן להוסיף את אותו URL, אירוע ותחום פעמיים.

לפני השמירה, לחצו על **Send Test Payload** כדי לבדוק שהנקודה מקבלת בקשה חתומה. ראו את הסעיף הבא, "Testing", לפרטים.

מהרשימה ניתן לערוך, להשבית, להפעיל מחדש או למחוק ווב-הוק. השבתה משאירה אירועים בתור עד שהווב-הוק מופעל מחדש; מחיקה מוחקת אותם.

ניתן גם ליצור ווב-הוקים דרך ה-API, לדוגמה באמצעות Zapier. הם מופיעים באותה רשימה עם המקור **API**. ראו Managing Webhooks via the API.

---