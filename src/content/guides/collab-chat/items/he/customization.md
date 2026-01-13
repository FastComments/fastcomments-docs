### תמיכה במצב כהה

### מצב כהה דינמי

אם מצב כהה באתר שלך נשלט על ידי הוספת מחלקת `.dark` לאלמנט body, ממשק המשתמש של Collab Chat יתאים זאת אוטומטית מבלי שיידרש אתחול מחדש. סגנונות הווידג'ט מעוצבים כדי להגיב לנוכחות מחלקה זו.

[inline-code-attrs-start title = 'דוגמת CSS למצב כהה'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/* CSS למצב כהה */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
[inline-code-end]

### עיצוב מותאם באמצעות CSS

ניתן להתאים את הופעת ההדגשות, חלונות הצ'אט ואלמנטים נוספים באמצעות CSS. הווידג'ט מוסיף מחלקות ספציפיות שניתן לטרגט בגליון הסגנונות שלך.

הדגשות הטקסט משתמשות במערכת עיצוב בועות ההערות של FastComments, לכן כל ההתאמות שהחלת על הווידג'ט התגובות הסטנדרטי ישפיעו גם על Collab Chat.

### התאמת הסרגל העליון

הסרגל העליון מציג את מספר המשתמשים המקוונים ואת מספר הדיונים. ניתן להתאים את מיקומו על ידי מתן אלמנט מותאם כ-`topBarTarget`:

[inline-code-attrs-start title = 'מיקום מותאם לסרגל העליון'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('my-custom-header')
});
[inline-code-end]

או להשבית אותו לחלוטין על ידי הגדרתו ל-`null`:

[inline-code-attrs-start title = 'השבתת הסרגל העליון'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});
[inline-code-end]

### התנהגות במובייל

במסכים ברוחב מתחת ל-768px, Collab Chat עובר אוטומטית לפריסת מותאמת לנייד. חלונות הצ'אט מופיעים במסך מלא במקום לרחף לצד הטקסט, ועיכוב הבחירה מוסר כדי לאפשר אינטראקציה מיידית יותר.

התנהגות זו מובנית ואינה דורשת קונפיגורציה. הווידג'ט מזהה את גודל המסך באופן אוטומטי ומתאים את עצמו בהתאם.

### מראה חלון הצ'אט

חלונות הצ'אט רוחבם 410px בדסקטופ עם חץ בגודל 16px שמצביע לטקסט המודגש. החלונות ממקמים את עצמם באופן אוטומטי לפי שטח הצפייה הזמין, באמצעות מחלקות מיקום כמו `to-right`, `to-left`, `to-top`, ו-`to-bottom`.

ניתן להוסיף CSS מותאם כדי להתאים צבעים, גופנים, ריווח או מאפייני חזות אחרים של החלונות הללו. חלונות הצ'אט משתמשים באותה מבנה רכיבים כמו הווידג'ט הסטנדרטי של FastComments, ולכן הם יורשים כל התאמה גלובלית שהחלת.

### לוקליזציה

Collab Chat תומך בכל אפשרויות הלוקליזציה אותן הווידג'ט הסטנדרטי של FastComments תומך. הגדר את האפשרות `locale` כדי להציג טקסט בממשק בשפות שונות:

[inline-code-attrs-start title = 'הגדרת locale'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // ספרדית
});
[inline-code-end]

FastComments תומכת בעשרות שפות. הגדרת ה-locale משפיעה על כל טקסט בממשק המשתמש, כולל הודעות, כפתורים וטקסטי מציין מקום.

### אפשרויות התאמה שיורשות

מכיוון ש-Collab Chat מרחיב את הווידג'ט התגובות הסטנדרטי, הוא יורש את כל אפשרויות ההתאמה מהווידג'ט הבסיסי. זה כולל מחלקות CSS מותאמות, תרגומים מותאמים, התאמת אווטאר, עיצוב תאריכים ועוד הרבה.

עיין בתיעוד ההתאמה הראשי של FastComments לרשימה המלאה של אפשרויות ההתאמה הזמינות.

### עבודה עם גופנים מותאמים

אם האתר שלך משתמש בגופנים מותאמים, ממשק המשתמש של Collab Chat יורש את אותם גופנים מקובץ ה-CSS של העמוד שלך. ייתכן שתצטרך ליצור כלל התאמה לווידג'ט ולייבא (`@import`) את כל הגופנים ב-CSS המותאם בכלל זה אם ברצונך
שחלון הצ'אט הצף ישתמש באותם גופנים.

---