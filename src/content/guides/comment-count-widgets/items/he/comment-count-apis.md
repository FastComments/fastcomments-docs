ישנן מספר נקודות קצה לקבלת הספירות, בהתאם למה שאתה רוצה ואם אתה רוצה לקבל אותן מדפדפן, שרת או באמצעות ה-API SDK.

## ספירות תגובות ציבוריות

אתה יכול לקבל את ספירות התגובות הציבוריות באמצעות הווידג'טים למעלה או באמצעות ה-APIs שהם משתמשים בהם. APIs אלה נשארו ללא שינוי מאז 2019 ולעולם לא ישתנו.

[inline-code-attrs-start title = 'Single Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count/:tenantId?urlId=page_id_or_url
[inline-code-end]

זה יחזיר מבנה כמו:

[inline-code-attrs-start title = 'Single Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{"count":0,"postfix":"comments"}
[inline-code-end]

המאפיין `postfix` תמיד כלול.

[inline-code-attrs-start title = 'Bulk Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count-bulk/:tenantId?urlIds=x,y,z
[inline-code-end]

זה יחזיר מבנה כמו:

[inline-code-attrs-start title = 'Bulk Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "translations": {
        "t174": "0 Comments",
        "t175": "1 Comment",
        "t176": "[count] Comments"
    },
    "counts": {
        "x": 10
    }
}
[inline-code-end]

האובייקט `counts` מאוכלס רק עבור דפים שיש להם ספירות. המפה `translations` תמיד נוכחת מכיוון שהיא משמשת לווידג'ט.

### התנהגות נקודות קצה ציבוריות / מטמון

לנקודות הקצה הציבוריות יש מנגנון מטמון של 60 שניות לטיפול בעומסי תנועה. באופן פנימי זהו מטמון LRU לכל תהליכון בזיכרון בשרת, כך שאתה עשוי לראות שהספירות משתנות מעט (עולות ואז יורדות זמנית) כאשר אנשים משאירים הרבה תגובות.

נקודות הקצה הציבוריות תמיד מחזירות את ספירת התגובות *הכוללת*, לא את ספירת תגובות השורש.

### APIs בצד השרת / SDK

הדרך לקבל תגובות מהשרת שלך היא לקרוא ל-[Pages API](/guide-api.html#page-structure) ולקבל את אובייקט הדף, שמכיל את ספירת התגובות הכוללת וספירת תגובות השורש. אנו מספקים SDKs שמאפשרים לך לקרוא ל-API זה מבלי לבנות את בקשת ה-API ידנית ומספקים ערכי החזרה מוקלדים.
