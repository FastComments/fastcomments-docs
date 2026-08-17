**Template ID:** `thread_summarizer`

ה‑Thread Summarizer מפרסם סיכום נייטרלי, בפסקה אחת, בסוף שרשור ארוך. הוא משתמש בעיכוב של 30 דקות כדי שהשרשור יתייצב לפני שהסוכן בודק אותו.

ה‑prompt המובנה מורה לסוכן לא לערוך – זה קריטי. ללא זאת המודל נוטה למסגור של "לדעתי" שמקבל מראה רע תחת שם התצוגה של החשבון שלך.

### Triggers

- **New comment posted** (`COMMENT_ADD`).
- **Trigger delay**: 30 minutes (1800 seconds). ראה [Deferred Triggers](#trigger-deferred-delay).

העיכוב של 30 דקות משמעותו שהסוכן פועל פעם אחת, חצי שעה אחרי שהתגובה נרשמה, על פי המצב של השרשור באותו רגע. זה לא "סכם על כל תגובה" – תור ה‑deferred‑trigger מאחד מספר אירועי תגובה חדשים באותו השרשור, אך אינו מסיר כפילויות בין חלונות זמן נפרדים. סביר להניח שתרצה **להוסיף כלל מותאם אישית ב‑prompt** כגון "אל תפרסם סיכום חדש אם הסוכן כבר סיכם את השרשור הזה ב‑24 השעות האחרונות" ולהסתמך על ההקשר יחד עם [כלי הזיכרון](#tools-overview) של הסוכן כדי לאכוף זאת.

### Allowed tools

- [`write_comment`](#tools-overview) – מפרסם את הסיכום עצמו.  
- [`pin_comment`](#tools-overview) – מצמיד את הסיכום כך שהקוראים יראו אותו בראש השרשור.  
- [`unpin_comment`](#tools-overview) – מסיר הצמדה של סיכום קודם על ידי אותו סוכן לפני הצמדה של החדש.

ה‑summarizer אינו יכול לבצע מודרציה או אינטראקציה עם משתמשים.

### Pinning the summary

הסוכן מפרסם תגובה חדשה עם `write_comment`, ואז קורא ל‑`pin_comment` עם מזהה התגובה שהוחזר. בריצות הבאות על אותו השרשור, ה‑prompt מורה לו לקרוא ל‑`unpin_comment` על הסיכום הקודם שלו תחילה – הפלטפורמה עצמה **אינה** מחייבת כלל של תגובה אחת מצומדת לכל שרשור, ולכן השארת הסיכום הקודם מצומד תגרום לשני סיכומים מצומדים זה לצד זה. סמן "Include parent comment and prior replies in the same thread" ב‑[Context Options](#context-options) כדי שהסוכן יוכל לראות את הסיכום המצומד הקודם.

### Recommended additions before going live

- **סמן "Include parent comment and prior replies in the same thread"** ב‑[Context Options](#context-options). summarizer ללא הקשר של השרשור הוא חסר תועלת.  
- **כוון את כלל גודל‑השרשור‑המינימלי.** "פחות מ‑5 תגובות" הוא ברירת המחדל של ה‑prompt, אך בקהילות פעילות 10‑20 הוא מתאים יותר. ערוך את ה‑prompt ישירות.  
- **הגבל לתבניות URL ספציפיות** אם אתה רוצה סיכומים רק בעמודים ארוכים, ולא בהודעות או דפי מוצר. ראה [Scope: URL and Locale Filters](#scope-url-locale).  
- **שמור על עלות.** סיכום הוא התבנית הצרכנית ביותר בטוקנים מכיוון שהיא קוראת את כל השרשור בכל ריצה. קבע [תקציב יומי](#budgets-overview) קפדני לפני הפעלת האפשרות ל‑Enabled.

### Avoiding repeat summaries

לסוכן יש גישה ל‑[`save_memory`](#tools-overview) ול‑[`search_memory`](#tools-overview) – ניתן להרחיב את ה‑prompt כדי להורות לו לתעד הערות מסוג "summarized {thread urlId}" ולבדוק אותן לפני פרסום נוסף. הזיכרון משותף לכל הסוכנים במנוי שלך.