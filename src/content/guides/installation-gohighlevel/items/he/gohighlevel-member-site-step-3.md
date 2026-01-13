עכשיו אנחנו הולכים ליצור את קוד FastComments המותאם עבורך. השתמש בעוזר למטה כדי להגדיר כיצד תרצה ש-FastComments יעבוד באתר GoHighLevel שלך:

[snippet id="gohighlevel-wizard"]

### סוגי תיבות תגובה

You can configure the `TYPE = 'commenting'` line to switch the product used (for example you can change it to `live` for streaming chat or `collab` for collab chat).

### הצבת תיבת התגובות במקום הרצוי

נניח שברצונך לשים תיבות תגובה בחלקים ספציפיים של הדף ולא במיקומים ברירת המחדל.
שנה שורה זו:

    const TARGET_ELEMENT_ID = ''; // קבע כדי להשתמש במצב div של היעד

אל:

    const TARGET_ELEMENT_ID = 'fc_box'; // קבע כדי להשתמש במצב div של היעד

לאחר מכן בעורך GHL, לחץ על כפתור "קוד" והוסף את המקום שבו אתה רוצה שהתגובות יופיעו:

[inline-code-attrs-start title = 'תיבת FastComments של GoHighLevel'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### סוג תיבת תגובות שונה לפי דף

נניח שברצונך שמשתמשים יסמנו וידונו בקטעי טקסט, או שישתמשו בממשק צ'אט שידור במקום זאת.

ראשית בצע את השלבים המופיעים לעיל ב'הצבת תיבת התגובות במקום הרצוי'.

שימו לב שבקטע הקוד הקטן יש `type="commenting"`.

לדוגמה, אם ברצונך להפעיל צ'אט collab, שנה את type ל- `type="collab"`.

### הצגה רק בעמודים ספציפיים

אם אינך מגדיר את `TARGET_ELEMENT_ID`, תוכל במקום זאת להגדיר את המשתנה `VALID_PATTERNS` כדי לקבוע באילו מסלולי URL תופענה התגובות. כברירת מחדל, הן יופיעו
בעמודים שמכילים `/post` ב-URL.

### הגדרת צ'אט Collab

אתה יכול להגדיר שצ'אט Collab יוסיף פונקציונליות שיתופית רק סביב HTML בתוך אזור מסוים. לדוגמה, נניח שהוספת את קוד ה-footer שלמעלה ואז הוספת את ה-div הזה בתוכן הפוסט/העמוד כדי לאפשר את צ'אט Collab:

[inline-code-attrs-start title = 'צאט Collab עם תוכן מוגדר'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

אז אלמנט הפסקה שבתוך ה-`<div>` יקבל צ'אט Collab מופעל, ושום דבר אחר בדף לא יושפע. אם לא
תכניס שום תוכן ל-`<div>` אז זה יפעיל את צ'אט Collab על כל גוף הפוסט.