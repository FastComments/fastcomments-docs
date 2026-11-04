כעת נייצר את קוד FastComments המותאם אישית שלך. השתמש באשף למטה כדי להגדיר כיצד אתה רוצה ש-FastComments יפעל באתר ה-GoHighLevel שלך:

[snippet id="gohighlevel-wizard"]

### סוגי תיבות תגובות שונים

ניתן להגדיר את השורה `TYPE = 'commenting'` כדי להחליף את המוצר בשימוש (למשל ניתן לשנות אותה ל-`live` עבור צ'אט שידור או `collab` עבור צ'אט שיתופי).

### הצבת תיבת התגובות במקום שתרצה

נניח שאתה רוצה להציב תיבות תגובות בחלקים ספציפיים של הדף ולא במיקומי ברירת המחדל.
שנה את השורה הזו:

    const TARGET_ELEMENT_ID = ''; // set to use target div mode

ל:

    const TARGET_ELEMENT_ID = 'fc_box'; // set to use target div mode

ואז בעורך GHL, לחץ על כפתור "code" והוסף את הקוד במקום שבו תרצה שהתגובות יופיעו:

[inline-code-attrs-start title = 'Div של GoHighLevel FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### סוג תיבת תגובות שונה לכל דף

נניח שאתה רוצה שמשתמשים יסמנו וידונו בחלקי טקסט, או שישתמשו בממשק צ'אט השידור במקום.

ראשית בצע את השלבים למעלה ב"הצבת תיבת התגובות במקום שתרצה".

שים לב שבקטע הקוד הקטן הזה יש `type="commenting"`.

אם אתה רוצה להפעיל למשל צ'אט שיתופי, שנה את ה-type ל-`type="collab"`.

### הצגה רק בדפים ספציפיים

אם לא תגדיר את `TARGET_ELEMENT_ID`, תוכל במקום זאת להגדיר את המשתנה `VALID_PATTERNS`, כדי לקבוע באילו נתיבי URL התגובות יוצגו. כברירת מחדל, הן יוצגו
בדפים שמכילים `/post` ב-URL.

### הגדרת צ'אט שיתופי

אתה יכול להורות לצ'אט השיתופי להוסיף פונקציונליות שיתופית רק סביב HTML בתוך אזור ספציפי. לדוגמה, נניח שאתה
מוסיף את קוד ה-footer למעלה ואז מוסיף את ה-div הזה בתוכן הפוסט/דף כדי להפעיל צ'אט שיתופי:

[inline-code-attrs-start title = 'צ\'אט שיתופי עם תוכן מוגדר'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

ואז רכיב הפסקה בתוך ה-`<div>` יכלול צ'אט שיתופי, ושום דבר אחר בדף לא יכלול זאת. אם לא
תציב תוכן כלשהו ב-`<div>`, הצ'אט השיתופי יופעל על כל גוף הפוסט.