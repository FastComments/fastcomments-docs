A **טריגר** הוא אירוע שמעיר סוכן. לכל סוכן יכול להיות מוגדר טריגר אחד או יותר.

### הרשימה המלאה

| טריגר | מתי הוא מתבצע |
|---|---|
| [Comment Added](#trigger-comment-add) | הודעה חדשה מתפרסמת. |
| [Comment Edited](#trigger-comment-edit) | הודעה נערכת. הטקסט הקודם נכלל בהקשר של הסוכן. |
| [Comment Deleted](#trigger-comment-delete) | הודעה נמחקת. |
| [Comment Pinned](#trigger-comment-pin) | הודעה מוצמדת (על ידי כל אחד, כולל מודרטור או סוכן אחר). |
| [Comment Unpinned](#trigger-comment-unpin) | הודעה מתבוטלת. |
| [Comment Locked](#trigger-comment-lock) | הודעה נעולה (אין אפשרות לתגובות נוספות). |
| [Comment Unlocked](#trigger-comment-unlock) | הודעה משוחררת. |
| [Comment Crosses Vote Threshold](#trigger-comment-vote-threshold) | מספר הקולות נטו של ההודעה מגיע לסף המוגדר. |
| [Comment Crosses Flag Threshold](#trigger-comment-flag-threshold) | מספר הדגלים של ההודעה מגיע בדיוק לסף המוגדר. |
| [User Posts First Comment](#trigger-new-user-first-comment) | משתמש מפרסם את ההודעה הראשונה שלו באתר זה. |
| [Comment Auto-Spammed](#trigger-comment-auto-spammed) | הודעה מסומנת אוטומטית כספאם על ידי מנוע הספאם. |
| [Moderator Reviews Comment](#trigger-moderator-reviewed) | מודרטור מסמן הודעה כנסקרת. |
| [Moderator Approves Comment](#trigger-moderator-approved) | מודרטור מאשר הודעה. |
| [Moderator Marks Spam](#trigger-moderator-spammed) | מודרטור מסמן הודעה כספאם. |
| [Moderator Awards Badge](#trigger-moderator-awarded-badge) | מודרטור מעניק תו למשתמש. |

### טריגרים מרובים לכל סוכן

סוכן יכול להירשם לכל שילוב של טריגרים - לדוגמה, תבנית [Moderator template](#template-moderator) נרשמת גם ל-`COMMENT_ADD` וגם ל-`COMMENT_FLAG_THRESHOLD`. כל אירוע מפעיל את הסוכן פעם אחת עם ההקשר של האירוע.

### מה מונע מהסוכן לפעול

אירוע טריגר שנרשם **לא** מפעיל את הסוכן אם מתקיימים אחד מהבאים:

- ה[מצב](#status-states) של הסוכן הוא **מושבת**.
- ה[כתובת URL או תחום השפה](#scope-url-locale) של הסוכן אינו תואם להודעה המפעילה.
- ה[תקציב היומי, החודשי או מגבלת הקצב](#budgets-overview) של הסוכן נגמר - הטריגר נרשם כ**נופל** עם סיבה. ראה [Drop Reasons](#drop-reasons).
- הקונקורנציה עבור סוכן זה רוויה (מוגבלת לכל סוכן).
- לשוכר של הסוכן יש חיוב לא תקין.
- הפעולה המפעילה בוצעה בעצמה על ידי בוט או סוכן אחר (מניעת לולאה).
- הטריגר היה עבור הודעה שכבר עובדה על ידי סוכן זה במסגרת חלון הדדופיקציה.

כאשר טריגר שנרשם מתבצע בהצלחה, ה[היסטוריית הריצה](#run-history) של הסוכן מציגה שורה עם סטטוס **Started** שמתקדם ל-**Success** או **Error** כאשר הריצה מסתיימת.

### סף קולות ודגלים

שני טריגרים - **Comment Crosses Vote Threshold** ו-**Comment Crosses Flag Threshold** - דורשים סף מספרי בטופס העריכה. הטריגר מתבצע ברגע שהספירה חוצה את הערך המוגדר (במיוחד, טריגר סף הדגלים מתבצע כאשר `flagCount === flagThreshold`, ולכן בחירה ב‑1 משמעותה "מתבצע על הדגל הראשון", ובחירה ב‑5 משמעותה "מתבצע כאשר מגיע הדגל החמישי").

### טריגרים מתוזמנים

כל טריגר ניתן לדחות כך שהסוכן ירוץ מאוחר יותר, לדוגמה לאחר שהקולות/דגלים/תגובות התייצבו. ראה [Deferred Triggers](#trigger-deferred-delay).

### מניעת לולאה

כדי למנוע לולאות אינסופיות, תגובות **שנכתבות על ידי סוכן** נושאות `botId`. טריגרים של תגובה חדשה מתעלמים מתגובות עם `botId`.

התוצאה: סוכנים יכולים לפעול בתגובה לפעולות *אנושיות* בשוכר שלך, אך פעולות שמקורן בסוכן לעולם אינן מפעילות טריגרים של סוכן. זה חל על כל סוגי הטריגרים.

### REPLAY: הטריגר הפנימי

קיים גם סוג טריגר פנימי `REPLAY` המשמש את תכונת [Test Runs (Replays)](#test-runs-replays). לא ניתן לבחור אותו בטופס העריכה - הוא קיים כדי שריצות חזרה יסומנו באופן מובחן בהיסטוריית הריצה ויוחרגו מתצוגות ריצה חיה.

---