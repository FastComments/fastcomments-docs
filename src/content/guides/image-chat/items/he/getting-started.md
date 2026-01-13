### מקרי שימוש

Image Chat מתאים מאוד לקבלת משוב על עיצוב כאשר צוותים צריכים לדון ברכיבים ספציפיים במוקאפים או בצילומי מסך. אתרי סקירות מוצרים יכולים לאפשר ללקוחות לדון בתכונות ספציפיות הנראות בתמונות המוצר. פלטפורמות חינוכיות יכולות להשתמש בו לדיון בתרשימים, מפות או תמונות מדעיות. גלריות תמונות יכולות להפוך לאינטראקטיביות עם הערות ממוקמות. אתרי נדל"ן יכולים לאפשר לצופים לדון בתכונות ספציפיות הנראות בתמונות הנכסים.

### התחלה מהירה

התחלה עם Image Chat פשוטה. אתה צריך את סקריפט FastComments Image Chat, אלמנט תמונה או מיכל עם תמונה, ואובייקט קונפיגורציה עם ה-Tenant ID שלך.

### התקנה

הוסף את סקריפט Image Chat לדף שלך:

[inline-code-attrs-start title = 'טעינת סקריפט Image Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### יישום בסיסי

להלן דוגמה מינימלית:

[inline-code-attrs-start title = 'יישום בסיסי של Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Image Gallery with Image Chat</title>
</head>
<body>
    <!-- התמונה שלך -->
    <img id="my-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Pretty Trail" />

    <!-- טען את סקריפט Image Chat -->
    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>

    <!-- אתחל את Image Chat -->
    <script>
        FastCommentsImageChat(document.getElementById('my-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Replace `'demo'` with your actual FastComments Tenant ID if it's not already, which you can find in your [FastComments dashboard](https://fastcomments.com/auth/my-account).

### איך זה עובד

ברגע שמאתחלים, משתמשים יכולים ללחוץ בכל מקום על התמונה. כשמתבצע קליק, מופיע סימן ריבועי חזותי באותו מיקום ונפתח חלון צ'אט. משתמשים אחרים יכולים לראות את כל הסימנים וללחוץ עליהם כדי לצפות או להשתתף בשיחות הללו. כל הדיונים מסונכרנים בזמן אמת בין כל המבקרים.

הווידג'ט משתמש במיקום מבוסס-אחוזים, כך שהסימנים נשארים במיקום הנכון גם כשהתמונה משתנה גודל או נצפית על גדלים שונים של מסכים.

### הדגמה חיה

אתה יכול לראות את Image Chat בפעולה בדף ההדגמה החיה שלנו: [live demo page](https://fastcomments.com/product/image-chat).

### הצעדים הבאים

עכשיו כשיש לך את הבסיס עובד, תוכל להתאים את המראה וההתנהגות במדריך אפשרויות התצורה. עיין במדריך העיצוב הרספונסיבי כדי להבין איך מיקום מבוסס-אחוזים פועל. למד על עיצוב ותמיכה במצב כהה במדריך ההתאמה האישית. לאינטגרציות מתקדמות, חקור את מדריך ה-API.

### ספריות צד לקוח

כל ספריות הפרונטאנד של FastComments (react, vue, angular, וכו') כוללות את Image Chat.

---