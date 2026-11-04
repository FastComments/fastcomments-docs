### סקירה כללית

FastComments Collab Chat מרחיב את ווידג'ט התגובות הרגיל של FastComments, ולכן הוא יורש את כל אפשרויות ההגדרה של הווידג'ט הבסיסי תוך הוספת מספר אפשרויות ייחודיות להערות טקסט.

### הגדרה נדרשת

#### tenantId

נדרש מזהה ה-Tenant שלך ב-FastComments. ניתן למצוא אותו ב-[לוח הבקרה של FastComments](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "דוגמת הגדרה"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### אפשרויות ייחודיות ל-Collab Chat

#### urlId

כברירת מחדל, Collab Chat מייצר מזהה ייחודי לכל שיחה בהתבסס על כתובת ה-URL של הדף, הנתיב ב-DOM לאלמנט, וטווח הטקסט המסומן. ניתן לדרוס זאת עם `urlId` מותאם אישית.

[inline-code-attrs-start title = "דוגמת הגדרה"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

זה שימושי כאשר מבנה ה-URL שלך עשוי להשתנות אך ברצונך לשמור על אותן שיחות, או כאשר ברצונך לשתף הערות בין מספר דפים.

#### topBarTarget

שולט בתצוגה של הסרגל העליון המציג את מספר המשתמשים ומספר הדיונים. הגדר ל-`null` כדי להשבית את הסרגל העליון לחלוטין, או ספק אלמנט DOM כדי להציג אותו במיקום ספציפי.

[inline-code-attrs-start title = "דוגמת הגדרה"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Disable top bar
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Render top bar in custom location
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

הפעל עיצוב מצב כהה כאשר לדף שלך יש רקע כהה. הזיהוי הזה אוטומטי, אך ייתכן שתרצה לדרוס אותו.

[inline-code-attrs-start title = "דוגמת הגדרה"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

פונקציית callback שמופעלת בכל פעם שמספר התגובות משתנה. זה שימושי לעדכון אלמנטים בממשק המשתמש כמו תגים או כותרות דף.

[inline-code-attrs-start title = "דוגמת הגדרה"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### אפשרויות הגדרה שהתקבלו בירושה

מכיוון ש-Collab Chat מרחיב את ווידג'ט התגובות הרגיל, ניתן להשתמש בכל אפשרות הגדרה מהווידג'ט הבסיסי של FastComments. הנה כמה אפשרויות בשימוש נפוץ:

#### locale

הגדר את שפת ממשק הווידג'ט. FastComments תומך בעשרות שפות.

[inline-code-attrs-start title = "דוגמת הגדרה"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spanish
});
[inline-code-end]

#### readonly

הפוך את כל השיחות לקריאה בלבד. משתמשים יכולים לצפות בהערות קיימות אך לא יכולים ליצור חדשות או להגיב.

[inline-code-attrs-start title = "דוגמת הגדרה"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

שלב עם מערכת האימות שלך באמצעות Single Sign-On.

[inline-code-attrs-start title = "דוגמת הגדרה"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO configuration
    }
});
[inline-code-end]

ראה את תיעוד ה-SSO לפרטים מלאים על אפשרויות האימות.

#### maxReplyDepth

שלוט בכמה רמות עומק תגובות יכולות להגיע. כברירת מחדל, Collab Chat מגדיר זאת ל-0, כלומר כל התגובות שטוחות (ללא תגובות מקוננות). ניתן לשנות זאת אם ברצונך בשיחות משורשרות.

[inline-code-attrs-start title = "דוגמת הגדרה"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Allow 3 levels of nesting
});
[inline-code-end]

### הגדרה פנימית

אפשרויות אלו מוגדרות אוטומטית על ידי Collab Chat ואין לדרוס אותן:

ה-`productId` מוגדר אוטומטית ל-`3` עבור Collab Chat. הרחבת ה-`floating-chat` נטענת אוטומטית כדי לספק את פונקציונליות חלון הצ'אט. הווידג'ט מזהה אוטומטית מכשירים ניידים (מסכים ברוחב פחות מ-768px) ומתאים את ממשק המשתמש בהתאם.

### דוגמה מלאה

הנה דוגמה המציגה מספר אפשרויות הגדרה יחד:

[inline-code-attrs-start title = "דוגמת הגדרה"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(document.getElementById('article'), {
    tenantId: 'demo',
    urlId: 'my-article-v2',
    hasDarkBackground: false,
    locale: 'en',
    topBarTarget: document.getElementById('custom-header'),
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) My Article` : 'My Article';
    },
    sso: {
        // Your SSO config
    },
    maxReplyDepth: 1
});
[inline-code-end]

לרשימה מלאה של כל אפשרויות ההגדרה הזמינות שהתקבלו בירושה מהווידג'ט הבסיסי, ראה את תיעוד ההגדרה הראשי של FastComments.