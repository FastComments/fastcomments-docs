**מזהה תבנית:** `thread_summarizer`

The Thread Summarizer מפרסם סיכום נייטרלי, פסקה אחת, בסוף שרשור ארוך. הוא משתמש בהשהייה של 30 דקות כדי לאפשר לשרשור להתייצב לפני שהסוכן בוחן אותו.

### הנחיה ראשונית מובנית

[inline-code-attrs-start title = 'הנחיה ראשונית לתבנית מסכם שרשור'; type='text' inline-code-attrs-end]
[inline-code-start]
You post neutral thread summaries. Do not summarize threads that have fewer than 5 comments. For longer threads, summarize the main positions, disagreements, and open questions in one short paragraph. Do not take sides and do not editorialize. After posting the summary, pin it. If a prior summary by you is already pinned on this thread, unpin it before pinning the new one.
[inline-code-end]

הוראת "do not editorialize" היא קריטית. בלעדיה המודל נוטה למסגר ב"אופן שבו אני רואה" שזה נשמע גרוע תחת שם התצוגה של החשבון שלכם.

### טריגרים

- **תגובה חדשה שנוספה** (`COMMENT_ADD`).
- **מרווח טריגר**: 30 דקות (1800 שניות). ראה [טריגרים מושהים](#trigger-deferred-delay).

ההשהייה של 30 דקות אומרת שהסוכן רץ פעם אחת, חצי שעה לאחר שנכנסת תגובה, על המצב שבו השרשור נמצא באותו רגע. זה אינו "לסכם על כל תגובה" — תור הטריגרים המושהים מאחד מספר אירועי תגובה-חדשה על אותו שרשור, אך אינו מבצע דה-דופליקציה בין חלונות זמן נפרדים. סביר שתרצו **להוסיף כלל מותאם בהנחיה שלכם** כמו "do not post a new summary if the agent has already summarized this thread in the last 24 hours" ולהסתמך על ההקשר יחד עם [כלי הזיכרון](#tools-overview) של הסוכן כדי לאכוף זאת.

### כלים מורשים

- [`write_comment`](#tools-overview) - מפרסם את הסיכום עצמו.
- [`pin_comment`](#tools-overview) - מקבע את הסיכום כדי שהקוראים יראו אותו בראש השרשור.
- [`unpin_comment`](#tools-overview) - מבודד סיכום קודם על-ידי אותו סוכן לפני שקיבעתם את החדש.

המסכם אינו יכול למתן או ליצור אינטראקציה עם משתמשים.

### קיבוע הסיכום

הסוכן מפרסם תגובה חדשה עם `write_comment`, ואז קורא ל-`pin_comment` עם מזהה התגובה שהוחזר. בהרצות עתידיות על אותו שרשור, ההנחיה מציינת שיתבצע קריאה ל-`unpin_comment` על הסיכום הקודם שלו קודם כל - הפלטפורמה עצמה אינה אוכפת כלל של תגובה יחידה מקובעת לכל שרשור, כך שמעבר לכך אם תשאירו את הסיכום הקודם מקובע, יתקבלו שני סיכומים מקובעים זה לצד זה. סמן/י "Include parent comment and prior replies in the same thread" ב[אפשרויות הקשר](#context-options) כדי שהסוכן יוכל לראות את הסיכום המקובע הקודם.

### תוספות מומלצות לפני הפעלה

- **סמן/י "Include parent comment and prior replies in the same thread"** ב[אפשרויות הקשר](#context-options). מסכם ללא הקשר לשרשור הוא חסר תועלת.
- **כוון/י את כלל המידע המינימלי של השרשור.** "פחות מ-5 תגובות" הוא ברירת המחדל של ההנחיה, אבל בקהילות עמוסות 10–20 מתאים יותר. ערכו את ההנחיה ישירות.
- **הגבל/י לדפוסי URL ספציפיים** אם ברצונכם סיכומים רק בעמודים ארוכי-תוכן, ולא בהודעות או בעמודי מוצר. ראה [היקף: מסנני URL ושפה/מיקום](#scope-url-locale).
- **היזהרו בעלויות.** סיכום הוא התבנית הצורכת הכי הרבה טוקנים כי היא קוראת את כל השרשור בכל הרצה. הגדירו [תקציב יומי](#budgets-overview) הדוק לפני שמפעילים מצב Enabled.

### הימנעות מסיכומים חוזרים

לסוכן יש גישה ל-[`save_memory`](#tools-overview) ו-[`search_memory`](#tools-overview) - ניתן להרחיב את ההנחיה כדי להנחות אותו לרשום הערות מסוג "סוכם {thread urlId}" ולבדוק אותן לפני פרסום נוסף. הזיכרון משותף לכל הסוכנים בתאנט שלכם.

---