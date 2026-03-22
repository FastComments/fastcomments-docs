הווידגט Recent Discussions מציג רשימת דפים ממוינת לפי פעילות התגובות האחרונה. הוא כולל כותרת, תאריכי פעילות אחרונים, ספירות תגובות עם אייקונים, וזיהוי מצב כהה אוטומטי.

## התקנה בסיסית

[inline-code-attrs-start title = 'התקנת ווידגט Recent Discussions'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

- **tenantId** (נדרש): מזהה השוכר (tenant) שלך ב-FastComments
- **count** (אופציונלי): מספר הדפים שיוצגו. ברירת מחדל `20`, מקסימום `100`
- **hasDarkBackground** (אופציונלי): לכפות עיצוב מצב כהה. מזוהה אוטומטית לפי רקע הדף אם לא הוגדר

## מבנה הווידגט

הווידגט מציג את מבנה ה-HTML הבא:

[inline-code-attrs-start title = 'מבנה HTML של ווידגט Recent Discussions'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

## CSS ברירת מחדל

[inline-code-attrs-start title = 'CSS ברירת מחדל של ווידגט Recent Discussions'; type = 'css'; isFunctional = false; inline-code-attrs-end]
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

## דוגמאות להתאמה

### הסר את גבול המיכל

[inline-code-attrs-start title = 'הסר את גבול המיכל'; type = 'css'; isFunctional = false; inline-code-attrs-end]
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

---