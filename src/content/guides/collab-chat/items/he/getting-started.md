### התחלה מהירה

התחלת העבודה עם Collab Chat פשוטה. אתם צריכים את סקריפט FastComments Collab Chat, אלמנט HTML המכיל את הטקסט שברצונכם להוסיף לו הערות, ואובייקט תצורה עם ה-Tenant ID שלכם.

### התקנה

הוסיפו את סקריפט Collab Chat לדף שלכם:

[inline-code-attrs-start title = 'טעינת סקריפט Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### מימוש בסיסי

הנה דוגמה מינימלית:

[inline-code-attrs-start title = 'מימוש בסיסי של Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- Your content container -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Load the Collab Chat script -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Initialize Collab Chat -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

החליפו את `'demo'` ב-Tenant ID האמיתי שלכם ב-FastComments אם זה עדיין לא כך, אותו תוכלו למצוא ב-[לוח הבקרה של FastComments](https://fastcomments.com/auth/my-account/api-secret).

### איך זה עובד

לאחר האתחול, משתמשים יכולים לסמן כל טקסט בתוך האלמנט הממוקד. לאחר השהיה קצרה (3.5 שניות בדסקטופ), מופיעה הודעה המאפשרת להם להתחיל דיון. כאשר נוצר דיון, מופיע סימון חזותי על הטקסט. משתמשים אחרים יכולים לרחף מעל הסימון או ללחוץ עליו כדי לצפות בדיון ולהשתתף בו. כל הדיונים מסתנכרנים בזמן אמת אצל כל המבקרים.

### הדגמה חיה

תוכלו לראות את Collab Chat בפעולה ב-[דף ההדגמה החיה](https://fastcomments.com/product/collab-chat) שלנו.

### הצעדים הבאים

עכשיו כשהבסיס עובד, תוכלו להתאים אישית את המראה וההתנהגות במדריך אפשרויות התצורה. עיינו במדריך התנהגות סימון טקסט כדי להבין כיצד פועל סימון טקסט. למדו על עיצוב ותמיכה במצב כהה במדריך ההתאמה האישית. לאינטגרציות מתקדמות, חקרו את ה-API Reference.

### ספריות Frontend

כל ספריות ה-Frontend של FastComments (react, vue, angular וכו') כוללות את Collab Chat.