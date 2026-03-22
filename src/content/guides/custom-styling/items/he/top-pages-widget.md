התוסף ווידג'ט "הדפים המדוברים ביותר" מציג רשימה מדורגת של הדפים שלך שקיבלו את מירב התגובות. הוא כולל כותרת, דירוגים ממוספרים, ספירות תגובות עם אייקונים, תאריכי פעילות אחרונים וזיהוי אוטומטי של מצב כהה.

## התקנה בסיסית

[inline-code-attrs-start title = 'התקנת ווידגט הדפים המדוברים ביותר'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-top-pages-v2.min.js"></script>
<div id="fastcomments-widget-top-pages"></div>
<script>
    FastCommentsTopPagesV2(document.getElementById('fastcomments-widget-top-pages'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## אפשרויות תצורה

- **tenantId** (נדרש): מזהה ה-tenant של FastComments שלך
- **hasDarkBackground** (אופציונלי): לכפות עיצוב למצב כהה. אם לא מוגדר, יזוהה אוטומטית מהרקע של הדף

## מבנה הווידג'ט

הווידג'ט מציג את מבנה ה-HTML הבא:

[inline-code-attrs-start title = 'מבנה ה-HTML של ווידגט הדפים המדוברים ביותר'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div class="fc-tp2">
    <div class="fc-tp2-heading">Most Discussed Pages</div>
    <div class="fc-tp2-list">
        <div class="fc-tp2-item">
            <div class="fc-tp2-rank">1</div>
            <div class="fc-tp2-detail">
                <a class="fc-tp2-title" href="...">Page Title</a>
                <span class="fc-tp2-activity">Last activity Mar 21, 2026</span>
            </div>
            <div class="fc-tp2-count">42</div>
        </div>
    </div>
</div>
[inline-code-end]

## הפניה ל-CSS ברירת מחדל

[inline-code-attrs-start title = 'CSS ברירת המחדל של ווידגט הדפים המדוברים ביותר'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-tp2 {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Helvetica Neue", sans-serif;
    text-align: left;
    line-height: 1.5;
    color: #1a1a1a;
    border: 1px solid #e0e0e0;
    border-radius: 12px;
    padding: 20px;
    background: #fff;
}
.fc-tp2-heading { font-size: 16px; font-weight: 700; margin-bottom: 14px; padding-bottom: 12px; border-bottom: 1px solid #eee; }
.fc-tp2-item { display: flex; align-items: center; gap: 12px; padding: 10px 0; border-bottom: 1px solid #f0f0f0; }
.fc-tp2-item:last-child { border-bottom: none; }
.fc-tp2-rank { width: 26px; height: 26px; display: flex; align-items: center; justify-content: center; border-radius: 50%; font-size: 11px; font-weight: 700; background: #f0f0f0; color: #888; }
.fc-tp2-title { font-size: 13px; font-weight: 500; color: #1a1a1a; text-decoration: none; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.fc-tp2-activity { font-size: 11px; color: #999; }
.fc-tp2-count { font-size: 12px; font-weight: 600; color: #666; }
[inline-code-end]

## דוגמאות להתאמה אישית

### הסר את תגי הדירוג

[inline-code-attrs-start title = 'הסר תגי הדירוג'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-tp2-rank {
    display: none !important;
}
[inline-code-end]

### הסר את גבול המיכל

[inline-code-attrs-start title = 'הסר את גבול המיכל'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-tp2 {
    border: none !important;
    box-shadow: none !important;
}
[inline-code-end]

---