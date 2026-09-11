Webhooks ניתן גם לנהל דרך ה-REST API. כך אינטגרציות כגון Zapier נרשמות
לאירועי תגובות מבלי לגעת בלוח הבקרה, והן פועלות לפי תבנית REST Hooks: הרשמה,
קבלת אירועים, ביטול הרשמה.

מנויים ב-API חיים לצד ה-webhooks המוגדרים בלוח הבקרה. אירוע תגובה נשלח
לכל webhook שתואם לדומיין שלו, כל אחד כהעברה נפרדת, ללא קשר לאופן שבו נוצר ה-webhook.

## Authentication

כל בקשה דורשת את מפתח ה-API שלך בכותרת `x-api-key` (או בפרמטר השאילתה `API_KEY`) ו
את מזהה השוכר שלך בפרמטר השאילתה `tenantId`. שני הפרטים מוצגים בעמוד API Secret בלוח הבקרה.

## Subscribe

```
POST https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
x-api-key: YOUR_API_KEY
Content-Type: application/json

{
    "url": "https://hooks.zapier.com/hooks/catch/123/abc",
    "event": "comment-created"
}
```

| שדה | נדרש | תיאור |
|-------|----------|-------------|
| `url` | כן | כתובת URL מוחלטת http או https. |
| `event` | כן | `comment-created`, `comment-updated` או `comment-deleted`. |
| `domain` | לא | דומיין מהגדרות החשבון שלך. ברירת המחדל היא `*`, שמקבל אירועים מכל דומיין. |
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

רישום של אותה כתובת URL לאותו אירוע ולדומיין שוב מחזיר את המנוי הקיים במקום
ליצור כפילות, כך שלקוח יכול לנסות שוב בבטחה. לכל שוכר ניתן עד 50 מנויים ב-API.

## List

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

מחזיר את כל ה-webhooks של השוכר, כולל אלו המנוהלים בלוח הבקרה (`"source": "dashboard"`).
ניתן לסנן באמצעות `event`, `domain` או `source`.

## Unsubscribe

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

מחיקת מנוי גם מסירה כל אירוע שעדיין בתור עבורו. רק מנויים שנוצרו
דרך ה-API ניתנים למחיקה באופן זה; webhook של לוח הבקרה, או מזהה שאינו קיים בחשבון שלך,
מחזיר `404` עם קוד `not-found`. ניתן לערוך webhooks של לוח הבקרה בעמוד Webhooks.

## Payloads and signing

ההעברות משתמשות באותו payload כמו webhooks של לוח הבקרה (ראו Data Structures) ונחתמות באותו
סכמת HMAC (ראו Security & API Tokens). מנויים ב-API לעולם לא מקבלים את הכותרת הישנה `token`,
לכן יש לאמת את הכותרת `X-FastComments-Signature` במקום זאת.

## Sample payloads

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

מחזיר את ההערות האחרונות של החשבון בדיוק במבנה שההעברה נושאת, כך שאינטגרציה יכולה
להציג נתוני דוגמה אמיתיים לפני שהאירוע הראשון מגיע. `event` הוא אופציונלי ונבדק בלבד,
מאחר שכל אירוע מעביר את אותו אובייקט תגובה. `limit` ברירת המחדל היא 3 ומקבל ערכים מ‑1 עד 10. עלות של 2 קרדיטים ב-API.

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

## Responding with 410 Gone

אם קצה של מנוי ב-API משיב עם HTTP `410 Gone`, FastComments מתייחס לכך כאל
ביטול מנוי: המנוי נמחק יחד עם האירועים בתור שלו, ולא מתבצעות עוד העברות.
Webhooks המוגדרים בלוח הבקרה אינם נמחקים אוטומטית; עבורם 410 הוא
כשל רגיל. כל קוד כשל אחר מנסה שוב ובסופו של דבר משבית את ה-webhook, כפי שמתואר
ב‑How it Works & Handling Retries.

## Dashboard

מנויים ב-API מופיעים ברשימת ה-Webhooks עם המקור **API**, שם מנהל יכול לערוך,
להשבית, להפעיל מחדש או למחוק אותם.