Webhooks יכולים גם להיות מנוהלים דרך ה‑REST API. כך אינטגרציות כגון Zapier נרשמות לאירועי תגובות מבלי לגעת בלוח הבקרה, והיא פועלת לפי תבנית REST Hooks: הרשמה, קבלת אירועים, ביטול הרשמה.

מנויים דרך ה‑API חיים לצד ה‑webhooks המוגדרים בלוח הבקרה. אירוע תגובה נשלח לכל webhook שתואם לדומיין שלו, כל אחד כהעברה נפרדת, ללא קשר לאופן שבו נוצר ה‑webhook.

## Authentication

כל בקשה דורשת את מפתח ה‑API שלך בכותרת `x-api-key` (או בפרמטר השאילתה `API_KEY`) ואת מזהה השוכר שלך בפרמטר השאילתה `tenantId`. שני הערכים מוצגים בעמוד API Secret בלוח הבקרה.

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
| `url` | כן | כתובת URL מוחלטת של http או https. |
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

רישום של אותה כתובת URL לאותו אירוע ולדומיין שוב מחזיר את המנוי הקיים במקום ליצור כפילות, ולכן לקוח יכול לנסות שוב בבטחה. לכל שוכר ניתן להחזיק עד 50 מנויים דרך ה‑API.

## List

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

מחזיר את כל ה‑webhook של השוכר, כולל אלו המנוהלים בלוח הבקרה (`"source": "dashboard"`). ניתן לסנן באמצעות `event`, `domain` או `source`.

## Unsubscribe

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

מחיקת מנוי גם מסירה כל אירוע שעדיין בתור עבורו. רק מנויים שנוצרו דרך ה‑API ניתנים למחיקה באופן זה. Webhooks בלוח הבקרה נערכים בעמוד Webhooks.

## Payloads and signing

ההעברות משתמשות באותו payload כמו webhooks בלוח הבקרה (ראו מבני נתונים) ונחתמות באותו סכמת HMAC (ראו אבטחה & אסימוני API). מנויים דרך ה‑API לעולם אינם מקבלים את הכותרת הישנה `token`, ולכן יש לאמת את הכותרת `X-FastComments-Signature` במקום זאת.

## Sample payloads

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

מחזיר את ההערות האחרונות של החשבון בדיוק במבנה שההעברה נושאת, כך שאינטגרציה יכולה להציג נתוני דוגמה אמיתיים לפני שהאירוע הראשון מגיע. `event` הוא אופציונלי ונבדק בלבד, מכיוון שכל אירוע מעביר את אותו אובייקט תגובה. `limit` ברירת המחדל היא 3 ומקבל ערכים מ‑1 עד 10. עלות של 2 קרדיטים של API.

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

אם קצה של מנוי API מגיב עם HTTP `410 Gone`, FastComments מתייחסת לכך כביטול מנוי: המנוי נמחק יחד עם האירועים בתור שלו, ולא מתבצעות עוד העברות. Webhooks המוגדרים בלוח הבקרה לעולם אינם נמחקים אוטומטית; עבורם 410 הוא כשל רגיל. כל סטטוס כשל אחר מנסה מחדש ובסופו של דבר משבית את ה‑webhook, כפי שמתואר ב‑How it Works & Handling Retries.

## Dashboard

מנויים דרך ה‑API מופיעים ברשימת ה‑Webhooks עם המקור **API**, שם מנהל יכול לערוך, להשבית, להפעיל מחדש או למחוק אותם.

---