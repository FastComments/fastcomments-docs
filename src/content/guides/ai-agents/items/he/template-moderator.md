---
**מזהה תבנית:** `tos_enforcer`

תבנית המודרטור היא נקודת ההתחלה המומלצת אם המטרה שלך היא להפחית את העומס על המודרציה הידנית. היא סוקרת תגובות חדשות ותגובות שסומנו ומיישמת את כללי הקהילה שלך.

### הנחיית התחלה מובנית

[inline-code-attrs-start title = 'הנחיית פתיחה של תבנית המודרטור'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a terms-of-service enforcement agent. Review each new comment against the community guidelines. Mark clear spam or policy violations. Issue warnings for first-offense borderline content. Escalate ban decisions only for repeat or severe violations. If a comment is clearly within the rules, approve it so it becomes visible (relevant for pre-moderation tenants). Stay out of political or subjective debates, focus on the rules as written.
[inline-code-end]

כמעט תמיד תרצה **להרחיב את ההנחיה הזו** עם דוגמאות קונקרטיות של מה שהאתר שלך מאפשר ומה שאינו מותר. מדיניות ההסלמה של הפלטפורמה עצמה (אזהרה לפני חסימה, חיפוש בזיכרון לפני חסימה) כבר מובנית בהנחיית המערכת שהסוכן מקבל, כך שאין צורך לחזור עליה.

### טריגרים

- **תגובה חדשה פורסמה** (`COMMENT_ADD`) - הסוכן בודק כל תגובה חדשה.
- **תגובה חצתה סף סימון** (`COMMENT_FLAG_THRESHOLD`, סף ברירת מחדל: 3) - הסוכן מעריך מחדש תגובה שמשתמשים אחרים סימנו.

### כלים מורשים

- [`mark_comment_approved`](#tools-overview) - שימושי עבור חשבונות בפיקוח מקדים שבהם הסוכן משחרר תגובות נקיות ומסתיר את השאר.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

הסוכן אינו יכול לפרסם תגובות, להצביע, להצמיד, לנעול, להעניק תגים, או לשלוח דוא"ל — ההנחיה מכווצת בכוונה.

### תוספות מומלצות לפני הפעלה

- **הגדר [הנחיות קהילה](#community-guidelines).** מספיק כמה משפטים של מדיניות כתובה; הסוכן ייישם אותה בכל הרצה.
- **חסום את `ban_user` מאחורי [אישור](#approval-workflow).** זה מופעל כברירת מחדל באזור ה-EU (ראה [ציות לסעיף 17 של DSA באיחוד האירופי](#eu-dsa-compliance)) ומומלץ בכל מקום.
- **שקול גם לחסום את `mark_comment_spam` מאחורי אישור** אם יש לך תוכן בנפח נמוך אך בעל השלכות גבוהות.
- **חסום את `mark_comment_approved` מאחורי אישור אם אתה מפעיל פיקוח מקדים.** אישור של תגובה מסוכנת מציג אותה בפני הקוראים; חסם את הפונקציה עד שהסוכן ירכוש אמון דרך הרצה יבשה.
- **סמנו "כלול גורם אמון של המגיב, גיל החשבון, היסטוריית חסימות, ותגובות אחרונות"** ב[אפשרויות הקשר](#context-options). המודל יזהר פחות כאשר יוכל לראות שמדובר במשתמש ותיק שפועל בתום לב.

### חלון הרצה יבשה מומלץ

הרץ תבנית זו ב[הרצה יבשה](#dry-run-mode) במשך לפחות שבוע נגד התעבורה האמיתית שלך לפני שתעביר למצב Enabled. השתמש ב[הרצות בדיקה (Replays)](#test-runs-replays) כדי גם לתצפת על התוצאות מול 30 הימים הקודמים.

---