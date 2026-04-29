**מזהה תבנית:** `tos_enforcer`

תבנית Moderator היא נקודת התחלה מומלצת אם המטרה שלך היא להפחית את עומס המודרציה הידנית. היא בודקת תגובות חדשות ומסומנות ומיישמת את כללי הקהילה שלך.

אתה כמעט תמיד תרצה **להרחיב את ה-prompt המובנה** עם דוגמאות קונקרטיות של מה שהאתר שלך מאפשר ומה שאינו מותר. מדיניות ההסלמה של הפלטפורמה עצמה (אזהרה לפני השעיה, חיפוש בזיכרון לפני השעיה) כבר משולבת בתוך ה-system prompt שהסוכן מקבל, כך שאין צורך לחזור עליה.

### טריגרים

- **New comment posted** (`COMMENT_ADD`) - הסוכן בוחן כל תגובה חדשה.
- **Comment crosses a flag threshold** (`COMMENT_FLAG_THRESHOLD`, default threshold: 3) - הסוכן מעריך מחדש תגובה שמשתמשים אחרים סימנו.

### כלים מורשים

- [`mark_comment_approved`](#tools-overview) - שימושי לחשבונות שמפעילים מודרציה מקדימה, שבהם הסוכן מפרסם תגובות נקיות ומסתיר את השאר.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

הוא לא יכול לפרסם תגובות, להצביע, להצמיד, לנעול, להעניק תגי הערכה או לשלוח אימייל - ה-prompt מצומצם בכוונה.

### תוספות מומלצות לפני הפעלה

- **Set [Community Guidelines](#community-guidelines).** כמה משפטים של מדיניות כתובה מספיקים; הסוכן מיישם אותה בכל הרצה.
- **Gate `ban_user` behind [approval](#approval-workflow).** זה מופעל כברירת מחדל באזור האיחוד האירופי (ראה [ציות למאמר 17 של DSA של האיחוד האירופי](#eu-dsa-compliance)) ומומלץ בכל מקום.
- **Consider also gating `mark_comment_spam` behind approval** אם יש לך תוכן בנפח נמוך אבל ברגישות גבוהה.
- **Gate `mark_comment_approved` behind approval if you run pre-moderation.** אישור של תגובה רעה מציב אותה בפני קוראים; חסום את האפשרות עד שהסוכן ירכוש אמון דרך הרצה ניסיונית.
- **Tick "Include commenter's trust factor, account age, ban history, and recent comments"** ב-[Context Options](#context-options). המודל יזהיר הרבה פחות ברוטאלית כשהוא יכול לראות שמדובר במשתמש ותיק ובהתנהגות טובה.

### חלון הרצה ניסיוני מומלץ

הרץ תבנית זו במצב [dry-run](#dry-run-mode) למשך לפחות שבוע נגד התנועה האמיתית שלך לפני שמחליפים ל-Enabled. השתמש ב-[Test Runs (Replays)](#test-runs-replays) כדי גם לצפות תצוגה מקדימה על פני 30 הימים הקודמים.