**מזהה תבנית:** `welcome_greeter`

ה‑Welcome Greeter עונה בחמימות למגיבים בפעם הראשונה. זוהי התבנית בעלת הסיכון הנמוך ביותר (ללא כלי פעולה הרסניים) וסוכן ראשון טוב להשקה בשידור חי.

### הנחיה ראשונית מובנית

[inline-code-attrs-start title = 'הנחיה ראשונית לתבנית Welcome Greeter'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a warm community greeter. Reply to first-time commenters with a short, personal welcome. Mention one specific thing from their comment so it does not read as a template. Keep replies to 1-2 sentences. Never reply to accounts more than 24 hours old.
[inline-code-end]

### טריגרים

- **משתמש חדש מפרסם את תגובתו הראשונה באתר זה** (`NEW_USER_FIRST_COMMENT`).

אירוע זה מופעל בדיוק פעם אחת לכל משתמש, לכן הסוכן לא יכול להיכנס ללופ. ראה [טריגר: תגובת משתמש חדש בפעם הראשונה](#trigger-new-user-first-comment).

### כלי עבודה מותרים

- [`write_comment`](#tools-overview)

זה הכלי היחיד - הסוכן פשוט לא יכול לבצע מודרציה, להצביע, לחסום, או לשלוח הודעה פרטית (DM).

### המלצות לתוספות לפני ההשקה בשידור חי

- **הגדר את שם התצוגה** למשהו מזמין - "Community Bot", הדמות המייצגת של האתר שלך, או שם המותג שלך. שם התצוגה הוא מה שהקוראים רואים מצורף לתשובת קבלת הפנים.
- **סמן "Include page title, subtitle, description, and meta tags"** ב[Context Options](#context-options). תגובות המברך משתפרות במידה ניכרת כשהיא יכולה להתייחס למה שהדף בעצם עוסק בו.
- **שקול הגבלות שפה/מיקום** אם אתה פועל במספר שפות. תשובת ברכה בשפה הלא נכונה צורמת יותר מתשובה שלא נשלחה. ראה [Scope: URL and Locale Filters](#scope-url-locale).

### מדוע אין צורך באישורים

הסוכן כותב רק תגובות חדשות ורק בעקבות טריגר חד-פעמי. במקרה הגרוע: ברכת פתיחה לא נוחה. אין פעולה הרסנית שדורשת אישור. רוב המפעילים מריצים את זה ללא אישורים כלל ברגע שההרצה היבשה נראית נקייה.

---