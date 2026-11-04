### תרחישי שימוש

Image Chat מתאים מצוין למשוב עיצובי, כאשר צוותים צריכים לדון באלמנטים ספציפיים במוקאפים או בצילומי מסך. אתרי סקירת מוצרים יכולים לאפשר ללקוחות לדון בתכונות ספציפיות הנראות בתמונות המוצר. פלטפורמות חינוכיות יכולות להשתמש בו לדיון בדיאגרמות, מפות או תמונות מדעיות. גלריות תמונות יכולות להפוך לאינטראקטיביות עם תגובות הקשורות למיקום. אתרי נדל"ן יכולים לאפשר למבקרים לדון בתכונות ספציפיות הנראות בתמונות הנכס.

### התחלה מהירה

התחלת העבודה עם Image Chat היא פשוטה. אתם צריכים את סקריפט FastComments Image Chat, אלמנט תמונה או מיכל עם תמונה, ואובייקט תצורה עם מזהה ה-Tenant שלכם.

### התקנה

הוסיפו את סקריפט Image Chat לדף שלכם:

[inline-code-attrs-start title = 'טעינת סקריפט Image Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### מימוש בסיסי

הנה דוגמה מינימלית:

[inline-code-attrs-start title = 'מימוש בסיסי של Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Image Gallery with Image Chat</title>
</head>
<body>
    <!-- Your image -->
    <img id="my-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Pretty Trail" />

    <!-- Load the Image Chat script -->
    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>

    <!-- Initialize Image Chat -->
    <script>
        FastCommentsImageChat(document.getElementById('my-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

החליפו את `'demo'` במזהה ה-Tenant האמיתי שלכם ב-FastComments אם זה עדיין לא המזהה שלכם, אותו תוכלו למצוא ב[לוח הבקרה של FastComments](https://fastcomments.com/auth/my-account).

### איך זה עובד

לאחר האתחול, משתמשים יכולים ללחוץ בכל מקום על התמונה. כאשר מתרחשת לחיצה, סמן ריבועי ויזואלי מופיע באותו מיקום וחלון צ'אט נפתח. משתמשים אחרים יכולים לראות את כל הסמנים וללחוץ עליהם כדי לצפות או להשתתף באותם דיונים. כל הדיונים מסתנכרנים בזמן אמת בקרב כל המבקרים.

הווידג'ט משתמש במיקום מבוסס אחוזים, כך שהסמנים נשארים במיקום הנכון גם כאשר התמונה משתנה בגודלה או נצפית בגדלי מסך שונים.

### הדגמה חיה

תוכלו לראות את Image Chat בפעולה ב[דף ההדגמה החיה](https://fastcomments.com/product/image-chat) שלנו.

### הצעדים הבאים

עכשיו כשיש לכם את היסודות עובדים, תוכלו להתאים אישית את המראה וההתנהגות במדריך אפשרויות תצורה. עיינו במדריך עיצוב רספונסיבי כדי להבין כיצד פועל מיקום מבוסס אחוזים. למדו על עיצוב ותמיכה במצב כהה במדריך ההתאמה האישית. לאינטגרציות מתקדמות, חקרו את ה-API Reference.

### ספריות Frontend

כל ספריות ה-FastComments ל-frontend (react, vue, angular וכו') כוללות את Image Chat.