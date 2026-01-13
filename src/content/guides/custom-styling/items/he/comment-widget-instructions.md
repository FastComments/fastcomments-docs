## כיצד להתאים אישית את סגנון הווידג'ט של התגובות

ניתן להתאים אישית את עיצוב הווידג'ט של התגובות בשתי דרכים:

### אפשרות 1: באמצעות פרמטר customCSS

העבר את ה-CSS המותאם אישית כמחרוזת לפרמטר `customCSS` בעת אתחול הווידג'ט:

```javascript
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: 'your-tenant-id',
    customCSS: `
        .fast-comments .comment {
            background-color: #f0f0f0 !important;
            border-radius: 8px !important;
        }
    `
});
```

### אפשרות 2: באמצעות לוח הניהול

1. עבור לדף [התאמת הווידג'ט](https://fastcomments.com/auth/my-account/customize-widget) בלוח הניהול שלך
2. גלול אל הסעיף "CSS מותאם אישית" תחת "מתקדם"
3. הזן את ה-CSS המותאם אישית שלך
4. לחץ על "שמור"

ה-CSS המותאם שלך יחול על כל הווידג'טים של התגובות באתר שלך.

## טיפים

- השתמש ב-`!important` כדי להחליף סגנונות ברירת מחדל במידת הצורך
- כוון סלקטורים ספציפיים כדי למנוע השפעה על חלקים אחרים באתר שלך
- בדוק את ה-CSS שלך בדפדפנים שונים כדי לוודא תאימות
- הווידג'ט משתמש ב-CSS סטנדרטי — אין צורך בכלי עיבוד מוקדם מיוחד