### דוגמה בסיסית

הדרך הפשוטה ביותר להשתמש ב-Image Chat היא לכוון לאלמנט תמונה יחיד. הדוגמה הזו מראה כיצד לאפשר דיונים אינטראקטיביים על תמונה:

[inline-code-attrs-start title = 'דוגמה בסיסית ל-Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>Product Image with Chat</title>
</head>
<body>
    <img id="product-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Product Photo" />

    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
    <script>
        FastCommentsImageChat(document.getElementById('product-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

### דוגמה עם אלמנט מכולה

אתה יכול גם להעביר אלמנט מכולה שיש בתוכו תמונה:

[inline-code-attrs-start title = 'Image Chat עם אלמנט מכולה'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<div id="image-container">
    <img src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="System Diagram" />
</div>

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('image-container'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

### דוגמה עם מזהה URL מותאם אישית

לפריסה ברירת המחדל, Image Chat משתמש ב-URL של הדף בשילוב עם מקור התמונה וקואורדינטות כדי לזהות שיחות. אתה יכול לספק `urlId` מותאם אישית:

[inline-code-attrs-start title = 'Image Chat עם מזהה URL מותאם אישית'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        urlId: 'product-v2-main-image'
    });
</script>
[inline-code-end]

זה שימושי אם מבנה ה-URL שלך משתנה אך אתה רוצה לשמור על אותן שיחות, או אם אתה רוצה לשתף את נקודות הדיון זהות על פני מספר דפים.

### דוגמה עם מצב כהה

אם לאתר שלך יש רקע כהה והווידג'ט לא מזהה זאת באופן אוטומטי כמו שצריך, ניתן להפעיל ידנית תמיכה במצב כהה:

[inline-code-attrs-start title = 'Image Chat עם מצב כהה'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

### דוגמה עם גודל ריבוע מותאם

אתה יכול להתאים את גודל הריבועים הניתנים ללחיצה שמופיעים על התמונה. הגודל מצוין כאחוז מרוחב התמונה:

[inline-code-attrs-start title = 'Image Chat עם גודל ריבוע מותאם'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>Image Chat with Custom Square Size</title>
</head>
<body>
    <img id="product-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Product Photo" />

    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
    <script>
        FastCommentsImageChat(document.getElementById('product-image'), {
            tenantId: 'demo',
            chatSquarePercentage: 2, // ריבועים קטנים יותר (ברירת מחדל היא 5)
        });
    </script>
</body>
</html>
[inline-code-end]

### דוגמה עם קריאת חזרה לספירת תגובות

עקוב מתי תגובות נוספות או מתעדכנות באמצעות הקריאה חזרה `commentCountUpdated`:

[inline-code-attrs-start title = 'Image Chat עם קריאת חזרה לספירת תגובות'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        commentCountUpdated: function(count) {
            console.log('Total comments:', count);
            document.getElementById('comment-badge').textContent = count;
        }
    });
</script>
[inline-code-end]

### דוגמה עם תמונות מרובות

אתה יכול לאתחל את Image Chat על מספר תמונות. לכל תמונה יהיו נקודות דיון עצמאיות משלה:

[inline-code-attrs-start title = 'Image Chat על תמונות מרובות'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<img id="image-1" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 1" />
<img id="image-2" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 2" />

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    // אתחול על התמונה הראשונה
    FastCommentsImageChat(document.getElementById('image-1'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-1'
    });

    // אתחול על התמונה השנייה
    FastCommentsImageChat(document.getElementById('image-2'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-2'
    });
</script>
[inline-code-end]

---