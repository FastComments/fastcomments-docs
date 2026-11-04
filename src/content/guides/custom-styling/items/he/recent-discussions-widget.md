הווידג'ט Recent Discussions מציג רשימה של עמודים ממוינים לפי פעילות התגובות האחרונה. הוא כולל כותרת, תאריכי פעילות אחרונה, מספרי תגובות עם אייקונים, וזיהוי אוטומטי של מצב כהה.

## התקנה בסיסית

[inline-code-attrs-start title = 'הגדרת עיצוב לווידג\'ט Recent Discussions'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

- **tenantId** (נדרש): מזהה ה-tenant שלך ב-FastComments
- **count** (אופציונלי): מספר העמודים להצגה. ברירת המחדל היא `20`, מקסימום `100`
- **hasDarkBackground** (אופציונלי): כופה עיצוב במצב כהה. מזוהה אוטומטית מרקע העמוד אם לא הוגדר

## מבנה הווידג'ט

הווידג'ט מוצג עם מבנה ה-HTML הבא:

[inline-code-attrs-start title = 'מבנה HTML של ווידג\'ט Recent Discussions'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div class="fc-rd2">
    <div class="fc-rd2-heading">Recent Discussions</div>
    <div class="fc-rd2-list">
        <div class="fc-rd2-item">
            <div class="fc-rd2-detail">
                <a class="fc-rd2-title" href="...">Page Title</a>
                <span class="fc-rd2-activity">Last activity Mar 21, 2026</span>
            </div>
            <div class="fc-rd2-count">42</div>
        </div>
    </div>
</div>
[inline-code-end]

## הפניה ל-CSS ברירת מחדל

[inline-code-attrs-start title = 'CSS ברירת מחדל של ווידג\'ט Recent Discussions'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rd2 {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Helvetica Neue", sans-serif;
    text-align: left;
    line-height: 1.5;
    color: #1a1a1a;
    border: 1px solid #e0e0e0;
    border-radius: 12px;
    padding: 20px;
    background: #fff;
}
.fc-rd2-heading { font-size: 16px; font-weight: 700; margin-bottom: 14px; padding-bottom: 12px; border-bottom: 1px solid #eee; }
.fc-rd2-item { display: flex; align-items: center; gap: 12px; padding: 10px 0; border-bottom: 1px solid #f0f0f0; }
.fc-rd2-item:last-child { border-bottom: none; }
.fc-rd2-title { font-size: 13px; font-weight: 500; color: #1a1a1a; text-decoration: none; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.fc-rd2-activity { font-size: 11px; color: #999; }
.fc-rd2-count { font-size: 12px; font-weight: 600; color: #666; }
[inline-code-end]

## דוגמאות התאמה אישית

### הסרת מסגרת המכולה

[inline-code-attrs-start title = 'הסרת מסגרת המכולה'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rd2 {
    border: none !important;
    box-shadow: none !important;
}
[inline-code-end]

### צבע קישור מותאם אישית

[inline-code-attrs-start title = 'צבע קישור מותאם אישית'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
a.fc-rd2-title:hover {
    color: #e63946 !important;
}
[inline-code-end]