ווידגט הדפים המובילים מציג רשימה של הדפים עם הכי הרבה תגובות.

ווידגט זה כולל עיצוב ברירת מחדל מינימלי ונועד להיות ניתן להתאמה בקלות באמצעות CSS משלך.

## מבנה הווידגט

הווידגט מציג את מבנה ה-HTML הבא:

```html
<div class="fastcomments-top-pages">
    <div class="page">
        <a class="title-link" href="...">Page Title (42)</a>
    </div>
    <!-- More pages... -->
</div>
```

## הפניה ל-CSS ברירת מחדל של הדפים המובילים

הווידגט כולל את עיצוב ברירת המחדל המינימלי הבא:

[inline-code-attrs-start title = 'CSS ברירת מחדל של ווידגט הדפים המובילים'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fastcomments-top-pages {
    font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen,Ubuntu,Cantarell,"Open Sans","Helvetica Neue",sans-serif;
}
.fastcomments-top-pages .page {
    padding-top: 5px;
}
[inline-code-end]

## דוגמאות להתאמה אישית

### הוספת עיצוב לקישורים
```css
.fastcomments-top-pages .title-link {
    color: #0066cc !important;
    text-decoration: none !important;
    font-size: 14px !important;
}

.fastcomments-top-pages .title-link:hover {
    text-decoration: underline !important;
}
```

### הוספת גבולות בין דפים
```css
.fastcomments-top-pages .page {
    border-bottom: 1px solid #eee !important;
    padding: 10px 0 !important;
}

.fastcomments-top-pages .page:last-child {
    border-bottom: none !important;
}
```

### עיצוב ספירת התגובות
```css
.fastcomments-top-pages .title-link {
    display: flex !important;
    justify-content: space-between !important;
}
```

---