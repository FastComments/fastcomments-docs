וידג'ט ספירת התגובות המרובות מעוצב להצגה יעילה של ספירות תגובות עבור דפים מרובים באותו דף. במקום לבצע קריאות API בודדות לכל ספירת תגובות, וידג'ט זה מקבץ בקשות לביצועים אופטימליים.

## התקנה בסיסית

[inline-code-attrs-start title = 'Bulk Comment Count Widget Installation'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<!-- Multiple elements with comment counts -->
<div class="fast-comments-count" data-fast-comments-url-id="page-1"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-2"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-3"></div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## איך זה עובד

הוידג'ט המרובה עובד על ידי:

1. סריקת הדף לאיתור אלמנטים עם המחלקה `fast-comments-count`
2. קריאת התכונה `data-fast-comments-url-id` מכל אלמנט
3. קיבוץ בקשות API לשליפת ספירות תגובות מרובות ביעילות
4. עדכון כל אלמנט עם ספירת התגובות המתאימה

## אפשרויות תצורה

הפונקציה `FastCommentsCommentCountBulk` מקבלת את אפשרויות התצורה הבאות:

- **tenantId** (נדרש): מזהה הדייר שלך ב-FastComments
- **apiHost** (אופציונלי): מארח API מותאם אישית אם אתה משתמש במופע מתארח עצמאית

## דוגמה מהעולם האמיתי

הנה דוגמה מעשית המראה כיצד תוכל להשתמש בוידג'ט המרובה ברשימת פוסטים בבלוג:

[inline-code-attrs-start title = 'Blog Post Listing with Comment Counts'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<style>
    .blog-post {
        border: 1px solid #ddd;
        margin: 10px 0;
        padding: 15px;
        border-radius: 5px;
    }
    .post-meta {
        color: #666;
        font-size: 14px;
        margin-top: 10px;
    }
    .comment-count {
        background: #e7f3ff;
        padding: 2px 8px;
        border-radius: 12px;
        font-size: 12px;
        display: inline-block;
    }
</style>

<div class="blog-post">
    <h3>How to Install FastComments</h3>
    <p>Learn how to add FastComments to your website in just a few minutes...</p>
    <div class="post-meta">
        Published: March 15, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="how-to-install-fastcomments"></span>
    </div>
</div>

<div class="blog-post">
    <h3>Advanced FastComments Configuration</h3>
    <p>Dive deep into the advanced configuration options for FastComments...</p>
    <div class="post-meta">
        Published: March 10, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="advanced-fastcomments-config"></span>
    </div>
</div>

<div class="blog-post">
    <h3>FastComments vs Other Solutions</h3>
    <p>See how FastComments compares to other commenting solutions...</p>
    <div class="post-meta">
        Published: March 5, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="fastcomments-comparison"></span>
    </div>
</div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## שיקולי ביצועים

הוידג'ט המרובה מייעל אוטומטית את הביצועים על ידי:

- **קיבוץ בקשות**: ספירות תגובות מרובות נשלפות בקריאת API יחידה
- **מגבלות גודל בקשה**: בקשות מתפצלות אוטומטית אם רשימת ה-URL הופכת גדולה מדי (מעל 1,000 תווים)
- **ביטול כפילויות**: אלמנטים מרובים עם אותו `data-fast-comments-url-id` חולקים את אותה ספירה

## אלמנטים מרובים עם אותו URL ID

אתה יכול לקבל אלמנטים מרובים בדף עם אותו `data-fast-comments-url-id`. כולם יעודכנו עם אותה ספירה:

[inline-code-attrs-start title = 'Multiple Elements Same URL ID'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<style>
    .count-example {
        margin: 10px 0;
        padding: 10px;
        background: #f9f9f9;
        border-radius: 5px;
    }
</style>

<div class="count-example">
    Header Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Sidebar Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Footer Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## לוקליזציה

הוידג'ט המרובה מעצב אוטומטית ספירות תגובות בהתבסס על הגדרות השפה שלך ב-FastComments. הוא מספק טקסט מתאים עבור:

- אפס תגובות
- תגובה אחת
- תגובות מרובות

## מתי להשתמש בוידג'ט מרובה לעומת וידג'ט בודד

**השתמש בוידג'ט המרובה כאשר:**
- יש לך ספירות תגובות מרובות באותו דף
- אתה מציג רשימת פוסטים/מאמרים עם ספירות תגובות
- הביצועים חשובים (מפחית קריאות API)

**השתמש בוידג'ט הבודד כאשר:**
- אתה צריך רק ספירת תגובות אחת בדף
- אתה צריך עדכונים חיים (הוידג'ט הבודד תומך בעדכונים בזמן אמת)
- אתה רוצה יותר שליטה על התנהגות הוידג'ט הבודד
