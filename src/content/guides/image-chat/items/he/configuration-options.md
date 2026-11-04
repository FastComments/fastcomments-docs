### סקירה כללית

FastComments Image Chat מרחיב את ווידג'ט התגובות הרגיל של FastComments, ולכן הוא יורש את כל אפשרויות ההגדרה של הווידג'ט הבסיסי תוך הוספת מספר אפשרויות ייחודיות להערות על תמונות.

### הגדרות נדרשות

#### tenantId

נדרש מזהה ה-Tenant שלך ב-FastComments. תוכל למצוא אותו ב-[לוח הבקרה של FastComments](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### אפשרויות ייחודיות ל-Image Chat

#### urlId

כברירת מחדל, Image Chat מייצר מזהה ייחודי לכל שיחה על בסיס כתובת ה-URL של הדף, מקור התמונה וקואורדינטות ה-X/Y. תוכל לדרוס זאת עם `urlId` מותאם אישית.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

זה שימושי כאשר מבנה ה-URL שלך עשוי להשתנות אך ברצונך לשמור על אותן שיחות, או כאשר ברצונך לשתף הערות בין מספר דפים.

#### chatSquarePercentage

שולט בגודל סמני הצ'אט הלחיצים כאחוז מרוחב התמונה. ברירת המחדל היא 5%, כלומר כל סמן הוא 5% מרוחב התמונה.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // סמנים גדולים ובולטים יותר
});
```

ערכים קטנים יוצרים סמנים פחות פולשניים שמתאימים יותר לתמונות מפורטות. ערכים גדולים מקלים על ראיית הסמנים ולחיצה עליהם בתמונות עמוסות או עבור משתמשים במכשירים ניידים.

#### hasDarkBackground

הפעלת עיצוב מצב כהה כאשר לדף שלך יש רקע כהה.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

פונקציית callback שמופעלת בכל פעם שמספר התגובות משתנה. זה שימושי לעדכון רכיבי ממשק משתמש כמו תגיות (badges) או כותרות דף.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### אפשרויות הגדרה שהתקבלו בירושה

מכיוון ש-Image Chat מרחיב את ווידג'ט התגובות הרגיל, תוכל להשתמש בכל אפשרות הגדרה מהווידג'ט הבסיסי של FastComments. הנה כמה אפשרויות בשימוש נפוץ:

#### locale

הגדרת שפת ממשק הווידג'ט. FastComments תומך בעשרות שפות.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // ספרדית
});
```

#### readonly

הפיכת כל השיחות לקריאה בלבד. משתמשים יכולים לצפות בסמנים ובדיונים קיימים אך אינם יכולים ליצור חדשים או להגיב.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso ו-simpleSSO

שילוב עם מערכת האימות שלך באמצעות Single Sign-On.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // הגדרות SSO
    }
});
```

ראה את תיעוד ה-SSO לפרטים מלאים על אפשרויות האימות.

#### maxReplyDepth

שליטה בעומק התגובות המקוננות. כברירת מחדל, Image Chat מגדיר זאת ל-0, כלומר כל התגובות שטוחות (ללא תגובות מקוננות). תוכל לשנות זאת אם ברצונך בשיחות משורשרות.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // אפשר 3 רמות של קינון
});
```

### הגדרות פנימיות

אפשרויות אלו מוגדרות אוטומטית על ידי Image Chat ואין לדרוס אותן:

ה-`productId` מוגדר אוטומטית ל-`2` עבור Image Chat. ההרחבה `floating-chat` נטענת אוטומטית כדי לספק את פונקציונליות חלון הצ'אט. הווידג'ט מזהה אוטומטית מכשירים ניידים (מסכים ברוחב פחות מ-768px) ומתאים את ממשק המשתמש בהתאם עם חלונות צ'אט במסך מלא.

### גמישות רכיב היעד

הפרמטר הראשון של `FastCommentsImageChat` יכול להיות רכיב `<img>` ישירות או רכיב מכל המכיל תמונה:

```javascript
// רכיב תמונה ישיר
FastCommentsImageChat(document.getElementById('my-image'), config);

// מכל עם תמונה בפנים
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

הווידג'ט ימצא את התמונה אוטומטית אם תעביר רכיב מכל.

### דוגמה מלאה

הנה דוגמה המציגה מספר אפשרויות הגדרה יחד:

```javascript
FastCommentsImageChat(document.getElementById('product-image'), {
    tenantId: 'demo',
    urlId: 'product-v2-main',
    chatSquarePercentage: 6,
    hasDarkBackground: false,
    locale: 'en',
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) Product Photo` : 'Product Photo';
    },
    sso: {
        // הגדרות ה-SSO שלך
    },
    maxReplyDepth: 1
});
```

לרשימה מלאה של כל אפשרויות ההגדרה הזמינות שהתקבלו מהווידג'ט הבסיסי, ראה את תיעוד ההגדרות הראשי של FastComments.