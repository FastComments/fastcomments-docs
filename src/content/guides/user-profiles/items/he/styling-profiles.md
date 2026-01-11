כאשר פרופילי משתמש נפתחים בהקשר של האתר שלך (דרך ווידג'ט התגובות), כל סגנונות CSS מותאמים שהחלת על הווידג'ט של FastComments מוזרקים אוטומטית לחלון הפרופיל.

### איך זה עובד

כאשר משתמש לוחץ על קישור לפרופיל מתוך ווידג'ט התגובות שלך, נפתח חלון פרופיל עם המחלקה `.fast-comments-profile`. ה-CSS המותאם של הווידג'ט שלך מוזרק אוטומטית לתצוגת הפרופיל. אם כבר עיצבת את ווידג'ט התגובות שלך, אותם סגנונות יחולו גם על הפרופילים.

### מחלקות CSS

פרופילי FastComments משתמשים בארכיטקטורת CSS מבוססת מחלקות. אין שימוש בתכונות מותאמות של CSS (CSS custom properties).

דף הפרופיל הראשי משתמש ב-`.user-profile` כמיכל השורש. מקטע הכותרת הוא `.profile-header` עם `.profile-header-background` לתמונת הרקע. תוכן הפרופיל נמצא ב-`.profile-content`.

האוואטר משתמש ב-`.profile-avatar` ו-`.profile-avatar-wrapper`. שם המשתמש הוא `.profile-name` וטקסט הביוגרפיה הוא `.profile-bio`. סטטיסטיקה נמצאת ב-`.profile-stats` עם סטטיסטיקות בודדות המשתמשות ב-`.stat`.

קישורים חברתיים נמצאים ב-`.profile-social-links` עם קישורים בודדים כ-`.social-link`. תגיות משתמש (badges) משתמשות ב-`.profile-badges` ו-`.badge`. פסי ההתקדמות של תגיות משתמש משתמשים ב-`.progress-outer` ו-`.progress-bar`.

טאבים משתמשים ב-`.profile-tabs` כמיכל, ב-`.tab` לטאבים בודדים, וב-`.tab.active` לטאב הנבחר. תוכן הטאב משתמש ב-`.tab-body` וב-`.tab-body.active`. ספירות ההתראות על טאבים משתמשות ב-`.tab .count`.

הודעות משתמשות ב-`.notification` ושיחות הודעות פרטיות (DM) משתמשות ב-`.conversation`. סטטוס מקוון הוא ב-`.activity-indicator` עם `.activity-indicator.online` למצב פעיל. מונים של הודעות לא נקראו משתמשים ב-`.unread-count`.

מיכל חלון הפרופיל הוא `.fast-comments-profile` עם `.fast-comments-profile-close` עבור כפתור הסגירה.

### מצב כהה

מצב כהה משתמש במודיפייר מחלקה `.dark` על `.user-profile`.

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### דוגמאות

**כותרת:**
```css
.user-profile .profile-header-background {
    background: linear-gradient(to right, #667eea, #764ba2);
}
```

**תגיות משתמש:**
```css
.user-profile .badge {
    background: #007bff;
    color: white;
    border-radius: 24px;
}
```

**טאבים:**
```css
.user-profile .tab.active {
    color: #007bff;
    border-bottom: 3px solid #007bff;
}
```

**חלון פרופיל:**
```css
.fast-comments-profile {
    border-radius: 12px 0 0 12px;
}
```