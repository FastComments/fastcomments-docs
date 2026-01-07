וידג'ט ספירת התגובות מעוצב להצגת מספר התגובות של דף יחיד. הוא קל משקל ומספק עדכונים בזמן אמת אם מוגדר.

## התקנה בסיסית

[inline-code-attrs-start title = 'Comment Count Widget Installation'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## אפשרויות תצורה

הפונקציה `FastCommentsCommentCount` מקבלת את אפשרויות התצורה הבאות:

- **tenantId** (נדרש): מזהה הדייר שלך ב-FastComments
- **urlId** (אופציונלי): מזהה הדף. ברירת מחדל היא `window.location.href` אם לא צוין
- **numberOnly** (אופציונלי): אם `true`, מציג רק את המספר ללא טקסט. ברירת מחדל היא `false`
- **isLive** (אופציונלי): אם `true`, הספירה תתעדכן אוטומטית. ברירת מחדל היא `false`

## דוגמאות מתקדמות

### URL ID מותאם אישית

[inline-code-attrs-start title = 'Comment Count with Custom URL ID'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-custom"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count-custom'), {
        tenantId: 'demo',
        urlId: 'my-custom-page-id'
    });
</script>
[inline-code-end]

### הצגת מספר בלבד

[inline-code-attrs-start title = 'Comment Count Number Only'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-number"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count-number'), {
        tenantId: 'demo',
        numberOnly: true
    });
</script>
[inline-code-end]

### עדכונים חיים

[inline-code-attrs-start title = 'Live Comment Count Updates'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-live"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count-live'), {
        tenantId: 'demo',
        isLive: true
    });
</script>
[inline-code-end]

## שיטות הוידג'ט

הוידג'ט מחזיר אובייקט עם השיטות הבאות:

- **destroy()**: מסיר את הוידג'ט ומנקה את כל הטיימרים
- **update(config)**: מעדכן את הוידג'ט עם תצורה חדשה

### דוגמת שימוש

[inline-code-attrs-start title = 'Widget Methods Example'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-methods"></div>
<script>
    const widget = window.FastCommentsCommentCount(document.getElementById('comment-count-methods'), {
        tenantId: 'demo'
    });

    // Update the widget to show a different page's count
    setTimeout(() => {
        widget.update({
            tenantId: 'demo',
            urlId: 'different-page-id'
        });
    }, 5000);

    // Destroy the widget after 10 seconds
    setTimeout(() => {
        widget.destroy();
    }, 10000);
</script>
[inline-code-end]

## עיצוב

הוידג'ט מרנדר HTML פשוט עם ספירת התגובות ומגיע עם עיצוב מינימלי. אתה יכול להתאים אישית את המראה עם CSS:

[inline-code-attrs-start title = 'Custom Styling'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<style>
    .comment-count-styled {
        background: #f0f0f0;
        padding: 5px 10px;
        border-radius: 15px;
        font-size: 14px;
        color: #666;
        display: inline-block;
    }
</style>
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-styled" class="comment-count-styled"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count-styled'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]
