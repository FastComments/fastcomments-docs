ווידגט תגובות אחרונות מציג רשימה של ההערות האחרונות באתר שלך או עבור עמוד מסוים. הוא כולל כותרת, תמונות פרופיל מעוגלות, תצוגות מקדימות של תגובות, תאריכים לחיצים שמקשרים ישירות להערה, וזיהוי אוטומטי של מצב כהה.

## התקנה בסיסית

[inline-code-attrs-start title = 'התקנת ווידגט תגובות אחרונות'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-comments-v2.min.js"></script>
<div id="fastcomments-widget-recent-comments"></div>
<script>
    FastCommentsRecentCommentsV2(document.getElementById('fastcomments-widget-recent-comments'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## אפשרויות תצורה

- **tenantId** (נדרש): מזהה ה-tenant של FastComments שלך
- **urlId** (אופציונלי): סינון לעמוד יחיד. השאר null לכל הדפים
- **count** (אופציונלי): מספר ההערות להצגה. ברירת המחדל היא `10`
- **hasDarkBackground** (אופציונלי): כפה עיצוב מצב כהה. אם לא מוגדר — ייתגלה אוטומטית לפי רקע העמוד

## מבנה הווידגט

הווידגט מציג את מבנה ה-HTML הבא:

[inline-code-attrs-start title = 'מבנה HTML של ווידגט תגובות אחרונות'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div class="fc-rc2">
    <div class="fc-rc2-heading">Recent Comments</div>
    <div class="fc-rc2-list">
        <div class="fc-rc2-card">
            <div class="fc-rc2-header">
                <img class="fc-rc2-avatar" src="..." alt="Avatar" />
                <div class="fc-rc2-meta">
                    <span class="fc-rc2-name">Username</span>
                    <a class="fc-rc2-date" href="...">2 hours ago</a>
                </div>
            </div>
            <div class="fc-rc2-body">Comment preview...</div>
            <a class="fc-rc2-page-link" href="...">Page Title</a>
        </div>
    </div>
</div>
[inline-code-end]

## CSS ברירת המחדל

[inline-code-attrs-start title = 'CSS ברירת המחדל של ווידגט תגובות אחרונות'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rc2 {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Helvetica Neue", sans-serif;
    text-align: left;
    line-height: 1.5;
    color: #1a1a1a;
    border: 1px solid #e0e0e0;
    border-radius: 12px;
    padding: 20px;
    background: #fff;
}
.fc-rc2-heading { font-size: 16px; font-weight: 700; margin-bottom: 14px; padding-bottom: 12px; border-bottom: 1px solid #eee; }
.fc-rc2-card { padding: 14px 0; border-bottom: 1px solid #f0f0f0; }
.fc-rc2-card:last-child { border-bottom: none; }
.fc-rc2-header { display: flex; align-items: center; gap: 10px; margin-bottom: 8px; }
.fc-rc2-avatar { width: 32px; height: 32px; border-radius: 50%; object-fit: cover; }
.fc-rc2-name { font-size: 13px; font-weight: 600; }
.fc-rc2-date { font-size: 11.5px; color: #999; text-decoration: none; }
.fc-rc2-body { font-size: 14px; line-height: 1.55; color: #444; display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; }
.fc-rc2-page-link { display: inline-block; margin-top: 6px; font-size: 12px; color: #777; text-decoration: none; }
.fc-rc2-page-link:hover { color: #0066cc; text-decoration: underline; }
[inline-code-end]

## דוגמאות להתאמה אישית

### שינוי גודל תמונת הפרופיל

[inline-code-attrs-start title = 'גודל תמונת פרופיל מותאם'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rc2-avatar {
    width: 40px !important;
    height: 40px !important;
}
[inline-code-end]

### הצג יותר שורות של תגובה

[inline-code-attrs-start title = 'הצג יותר שורות של תגובה'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rc2-body {
    -webkit-line-clamp: 5 !important;
}
[inline-code-end]

### הסר את גבול המכולה

[inline-code-attrs-start title = 'הסר את גבול המכולה'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rc2 {
    border: none !important;
    box-shadow: none !important;
}
[inline-code-end]

---