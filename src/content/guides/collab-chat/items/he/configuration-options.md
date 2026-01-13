### סקירה

FastComments Collab Chat מרחיב את ווידג'ט התגובות הסטנדרטי של FastComments, כך שהוא יורש את כל אפשרויות התצורה מהווידג'ט הבסיסי ומוסיף כמה אפשרויות ספציפיות להערות טקסט.

### תצורה נדרשת

#### tenantId

נדרש מזהה השוכר (Tenant ID) של FastComments שלך. ניתן למצוא אותו ב[לוח הבקרה של FastComments](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "דוגמת תצורה"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### אפשרויות ספציפיות ל-Collab Chat

#### urlId

ברירת המחדל היא ש-Collab Chat מייצר מזהה ייחודי לכל שיחה בהתבסס על כתובת הדף, נתיב ה-DOM לאלמנט וטווח הטקסט שנבחר. ניתן לעקוף זאת עם `urlId` מותאם אישית.

[inline-code-attrs-start title = "דוגמת תצורה"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

זה שימושי כאשר מבנה ה-URL שלך עשוי להשתנות ואתה רוצה לשמור על אותן שיחות, או כאשר אתה רוצה לשתף הערות על פני מספר דפים.

#### topBarTarget

שולט בתצוגת הסרגל העליון שמראה את מספר המשתמשים ומספר הדיונים. הגדר כ-`null` כדי להשבית את הסרגל העליון לחלוטין, או ספק אלמנט DOM כדי להציג אותו במיקום ספציפי.

[inline-code-attrs-start title = "דוגמת תצורה"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// השבתת סרגל עליון
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// הצג את הסרגל העליון במיקום מותאם אישית
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

הפעל סגנון מצב כהה כשלעמוד שלך יש רקע כהה. זיהוי זה אוטומטי, אך ייתכן שתרצה לעקוף אותו.

[inline-code-attrs-start title = "דוגמת תצורה"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

פונקציית החזרה שמופעלת בכל פעם שמספר התגובות משתנה. זה שימושי לעדכון רכיבים בממשק המשתמש כמו סיכות או כותרות עמוד.

[inline-code-attrs-start title = "דוגמת תצורה"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### אפשרויות תצורה שנירשו

מכיוון ש-Collab Chat מרחיב את ווידג'ט התגובות הסטנדרטי, ניתן להשתמש בכל אפשרות תצורה מהווידג'ט הבסיסי של FastComments. הנה כמה אפשרויות נפוצות:

#### locale

קבע את שפת ממשק הווידג'ט. FastComments תומך בעשרות שפות.

[inline-code-attrs-start title = "דוגמת תצורה"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // ספרדית
});
[inline-code-end]

#### readonly

הפוך את כל השיחות לקריאה בלבד. משתמשים יכולים לצפות בהערות קיימות אך לא יכולים ליצור חדשות או להגיב.

[inline-code-attrs-start title = "דוגמת תצורה"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

שלב עם מערכת האימות שלך באמצעות Single Sign-On.

[inline-code-attrs-start title = "דוגמת תצורה"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // תצורת SSO
    }
});
[inline-code-end]

עיין בתיעוד SSO לפרטים מלאים על אפשרויות האימות.

#### maxReplyDepth

שלוט כמה רמות עומק תגובות יכולות להיות. כברירת מחדל, Collab Chat מגדיר זאת כ-0, כלומר כל התגובות שטוחות (ללא תגובות מקוננות). ניתן לשנות זאת אם ברצונך לשוחח בשרשור תגובות.

[inline-code-attrs-start title = "דוגמת תצורה"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // אפשר 3 רמות קינון
});
[inline-code-end]

### תצורה פנימית

אופציות אלה מוגדרות אוטומטית על ידי Collab Chat ולא יש להן לעקוף אותן:

ה-`productId` מוגדר אוטומטית ל-`3` עבור Collab Chat. התוסף `floating-chat` נטען אוטומטית כדי לספק את פונקציונליות חלון הצ'אט. הווידג'ט מזהה אוטומטית מכשירי מובייל (מסכים ברוחב פחות מ-768px) ומתאים את הממשק בהתאם.

### דוגמה מלאה

הנה דוגמה המציגה מספר אפשרויות תצורה יחד:

[inline-code-attrs-start title = "דוגמת תצורה"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
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
        // תצורת SSO שלך
    },
    maxReplyDepth: 1
});
[inline-code-end]

למידע מלא על כל אפשרויות התצורה הזמינות שנירשות מהווידג'ט הבסיסי, עיין בתיעוד התצורה הראשי של FastComments.