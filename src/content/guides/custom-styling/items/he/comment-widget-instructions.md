## כיצד להתאים את סגנונות ווידג'ט התגובות

ניתן להתאים את עיצוב ווידג'ט התגובות בשתי דרכים:

### אפשרות 1: באמצעות הפרמטר `customCSS`

העבר את ה-CSS המותאם שלך כמחרוזת לפרמטר `customCSS` בעת אתחול הווידג'ט:

```javascript
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: 'your-tenant-id',
    customCSS: `
        .fast-comments .comment {
            background-color: #f0f0f0 !important;
            border-radius: 8px !important;
        }
    `
}];
```

### אפשרות 2: באמצעות לוח הניהול

1. גש ל[דף התאמת הווידג'ט](https://fastcomments.com/auth/my-account/customize-widget) בלוח הניהול שלך  
2. גלול אל מקטע "CSS מותאם" תחת "מתקדם"  
3. הזן את ה-CSS המותאם שלך  
4. לחץ על "שמור"

ה-CSS המותאם שלך יחול על כל הווידג'טים של התגובות באתר שלך.

## טיפים

- השתמש ב-`!important` כדי לעקוף סגנונות ברירת מחדל במידת הצורך  
- כוון סלקטורים ספציפיים כדי למנוע השפעה על חלקים אחרים באתר שלך  
- בדוק את ה-CSS שלך בדפדפנים שונים עבור תאימות  
- הווידג'ט משתמש ב-CSS סטנדרטי — אין צורך במעבדי CSS מיוחדים