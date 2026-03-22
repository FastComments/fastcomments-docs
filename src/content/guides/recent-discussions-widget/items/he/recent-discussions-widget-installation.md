ווידג'ט דיונים אחרונים מציג עמודים באתר שלך שיש בהם את פעילות התגובות האחרונה. כל רשומה מציגה את כותרת העמוד, תאריך הפעילות האחרון וסך כל התגובות. הוא מזהה באופן אוטומטי רקעים כהים ומתאים את העיצוב שלו בהתאם.

## התקנה בסיסית

[inline-code-attrs-start title = 'התקנת ווידגט דיונים אחרונים'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## אפשרויות תצורה

הפונקציה `FastCommentsRecentDiscussionsV2` מקבלת את אפשרויות התצורה הבאות:

- **tenantId** (required): מזהה ה-tenant של FastComments שלך
- **count** (optional): מספר העמודים להצגה. ברירת המחדל היא `20`, מקסימום `100`
- **hasDarkBackground** (optional): כפה עיצוב במצב כהה. יזוהה אוטומטית מרקע הדף אם לא הוגדר

## דוגמאות מתקדמות

### ספירה מותאמת

[inline-code-attrs-start title = 'דיונים אחרונים עם ספירה מותאמת'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        count: 5
    });
</script>
[inline-code-end]

### כפיית מצב כהה

[inline-code-attrs-start title = 'דיונים אחרונים במצב כהה'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

---