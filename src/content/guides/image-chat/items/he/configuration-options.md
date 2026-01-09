### Overview

FastComments Image Chat מרחיב את ווידג'ט התגובות הסטנדרטי של FastComments, כך שהוא יורש את כל אפשרויות התצורה מהווידג'ט הבסיסי תוך הוספת כמה אפשרויות ייעודיות להערות על תמונות.

### Required Configuration

#### tenantId

נדרש ה-Tenant ID של FastComments שלך. ניתן למצוא זאת ב-[FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Image Chat Specific Options

#### urlId

ברירת המחדל, Image Chat מייצר מזהה ייחודי לכל שיחה בהתבסס על כתובת הדף, מקור התמונה וקואורדינטות X/Y. ניתן להחליף זאת ב-`urlId` מותאם אישית.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

זה שימושי כאשר מבנה ה-URL שלך עשוי להשתנות אך אתה רוצה לשמור על אותן שיחות, או כאשר אתה רוצה לשתף הערות בין כמה דפים.

#### chatSquarePercentage

שולט בגודל הסמנים הניתנים ללחיצה כאחוז מרוחב התמונה. ברירת המחדל היא 5%, כלומר כל סמן הוא 5% מרוחב התמונה.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // סמנים גדולים יותר וברורים יותר
});
```

ערכים קטנים יותר יוצרים סמנים פחות חודרניים שעובדים טוב יותר לתמונות מפורטות. ערכים גדולים יותר מקלים על ראייה ולחיצה על תמונות עמוסות או עבור משתמשים במכשירים ניידים.

#### hasDarkBackground

הפעל עיצוב מצב כהה כאשר לעמוד שלך יש רקע כהה.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

פונקציית callback שמתבצעת בכל פעם שמספר ההערות משתנה. זה שימושי לעדכון אלמנטים בממשק המשתמש כמו סמלונים או כותרות דפים.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Inherited Configuration Options

מכיוון ש-Image Chat מרחיב את ווידג'ט התגובות הסטנדרטי, ניתן להשתמש בכל אפשרות תצורה מהווידג'ט הבסיסי של FastComments. הנה כמה אפשרויות נפוצות:

#### locale

קבע את השפה לממשק הווידג'ט. FastComments תומכת בעשרות שפות.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // ספרדית
});
```

#### readonly

הפוך את כל השיחות לקריאה בלבד. משתמשים יכולים לצפות בסמנים ובדיונים הקיימים אך לא יכולים ליצור חדשים או להשיב.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso and simpleSSO

שלב עם מערכת האימות שלך באמצעות Single Sign-On.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // תצורת SSO
    }
});
```

עיין בתיעוד ה-SSO לפרטים מלאים על אפשרויות האימות.

#### maxReplyDepth

שלוט בכמה רמות עמוקות התגובות יכולות להגיע. כברירת מחדל, Image Chat מקבע זאת ל-0, כלומר כל ההערות שטוחות (לא תגובות מקוננות). ניתן לשנות זאת אם ברצונך שיחות בשרשור.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // אפשר 3 רמות קיבוץ
});
```

### Internal Configuration

אפשרויות אלה מוגדרות אוטומטית על ידי Image Chat ולא יש לשנות אותן:

The `productId` is automatically set to `2` for Image Chat. The `floating-chat` extension is automatically loaded to provide the chat window functionality. The widget automatically detects mobile devices (screens under 768px wide) and adjusts the UI accordingly with fullscreen chat windows.

### Target Element Flexibility

הפרמטר הראשון ל-`FastCommentsImageChat` יכול להיות או אלמנט `<img>` ישירות או אלמנט מיכל עם תמונה בפנים:

```javascript
// אלמנט תמונה ישיר
FastCommentsImageChat(document.getElementById('my-image'), config);

// מיכל עם תמונה בתוך
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

הווידג'ט ימצא את התמונה אוטומטית אם תעביר אלמנט מיכל.

### Complete Example

הנה דוגמה שמציגה כמה אפשרויות תצורה יחד:

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
        // תצורת ה-SSO שלך
    },
    maxReplyDepth: 1
});
```

For a complete list of all available configuration options inherited from the base widget, see the main FastComments configuration documentation.