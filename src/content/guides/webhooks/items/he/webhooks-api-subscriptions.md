ניתן לנהל Webhooks גם דרך ה-REST API. כך מתממשקויות כמו Zapier נרשמות לאירועי תגובות מבלי לגעת בלוח הבקרה, והדבר עוקב אחר תבנית REST Hooks: הרשמה, קבלת אירועים, ביטול הרשמה.

מנויי API חיים לצד ה-webhooks המוגדרים בלוח הבקרה. אירוע תגובה נשלח לכל webhook שתואם לדומיין שלו, כל אחד כמשלוח נפרד, באשר דרך שנוצר ה-webhook.

## אימות

כל בקשה דורשת את מפתח ה-API שלך בכותרת `x-api-key` (או בפרמטר השאילתה `API_KEY`) ואת מזהה ה-tenant שלך בפרמטר השאילתה `tenantId`. שניהם מוצגים בדף API Secret בלוח הבקרה.

## הרשמה

```
POST https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
x-api-key: YOUR_API_KEY
Content-Type: application/json

{
    "url": "https://hooks.zapier.com/hooks/catch/123/abc",
    "event": "comment-created"
}
```

| שדה | חובה | תיאור |
|-------|----------|-------------|
| `url` | כן | כתובת URL מוחלטת של http או https. |
| `event` | כן | `comment-created`, `comment-updated` או `comment-deleted`. |
| `domain` | לא | דומיין מהגדרות החשבון שלך. ברירת המחדל היא `*`, שמקבל אירועים עבור כל דומיין. |
| `method` | לא | `POST` (ברירת מחדל), `PUT` או `DELETE`. |

התשובה מכילה את המנוי:

```json
{
    "status": "success",
    "webhook": {
        "id": "66f1c4c1e7a2b3d4f5a6b7c8",
        "url": "https://hooks.zapier.com/hooks/catch/123/abc",
        "event": "comment-created",
        "domain": "*",
        "method": "POST",
        "source": "api",
        "enabled": true,
        "createdAt": "2026-09-08T12:00:00.000Z"
    }
}
```

הרשמה של אותה כתובת URL לאותו אירוע ודומיין שוב מחזירה את המנוי הקיים במקום ליצור כפילות, כך שלקוח יכול לנסות שוב בבטחה. לכל tenant יכולים להיות עד 50 מנויי API.

## רשימה

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

מחזיר כל webhook עבור ה-tenant, כולל אלה המנוהלים בלוח הבקרה (`"source": "dashboard"`). ניתן לסנן לפי `event`, `domain` או `source`.

## ביטול הרשמה

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

מחיקת מנוי משליכה גם את כל האירועים שעדיין ממתינים בתור עבורו. רק מנויים שנוצרו דרך ה-API ניתנים למחיקה בדרך זו; webhook של לוח בקרה, או מזהה שאינו קיים בחשבונך, מחזירים `404` עם קוד `not-found`. Webhooks של לוח הבקרה נערכים בדף Webhooks.

## מטענים וחתימה

משלוחים משתמשים באותו מטען כמו webhooks של לוח הבקרה (ראה Data Structures) וחתומים באותה סכימת HMAC (ראה Security & API Tokens). מנויי API לעולם לא מקבלים את כותרת ה-`token` המיושנת, ולכן יש לאמת את הכותרת `X-FastComments-Signature` במקום זאת.

## מטענים לדוגמה

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

מחזיר את התגובות האחרונות בחשבון בצורה המדויקת שבה מטען משלוח נושא, כך שמתממשקות יכולות להציג נתוני דוגמה אמיתיים לפני הגעת האירוע הראשון. `event` הוא אופציונלי ומאומת בלבד, מכיוון שכל אירוע משלח את אותו אובייקט תגובה. `limit` ברירת המחדל היא 3 ומקבל 1 עד 10. עולה 2 נקודות API.

```json
{
    "status": "success",
    "payloads": [
        {
            "id": "66f1c4c1e7a2b3d4f5a6b7c8",
            "urlId": "https://example.com/blog/hello-world",
            "commenterName": "Jane Reader",
            "comment": "Great article!",
            "date": "2026-09-08T12:00:00.000Z",
            "approved": true
        }
    ]
}
```

## תגובה עם 410 Gone

אם נקודת הקצה של מנוי API מגיבה עם HTTP `410 Gone`, FastComments מתייחס לכך כביטול הרשמה: המנוי נמחק יחד עם האירועים הממתינים בתור שלו, ולא מנסים משלוחים נוספים. Webhooks המוגדרים בלוח הבקרה לעולם לא נמחקים אוטומטית; עבורם 410 הוא כישלון רגיל. כל סטטוס כישלון אחר מנוסה שוב ובסופו של דבר משבית את ה-webhook, כפי שמתואר ב-How it Works & Handling Retries.

## לוח הבקרה

מנויי API מופיעים ברשימת Webhooks עם המקור **API**, שם מנהל יכול לערוך, להשבית, להפעיל מחדש או למחוק אותם.