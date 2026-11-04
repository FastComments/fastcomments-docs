עמוד הגדרות התוסף נמצא ב-**Site Administration > Plugins > Local plugins > FastComments**. האפשרויות הזמינות הן:

#### Tenant ID

מזהה ה-Tenant שלך ב-FastComments. ניתן למצוא זאת ב-<a href="https://fastcomments.com/auth/my-account" target="_blank">לוח הבקרה של FastComments</a> תחת הגדרות החשבון שלך.

#### API Secret

מפתח ה-API Secret שלך, הנדרש למצב Secure SSO. ניתן למצוא זאת ב-<a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a>.

#### SSO Mode

בחר כיצד משתמשים מאומתים. ראה את הסעיף [SSO Modes](#moodle-sso-modes) לפרטים על כל אפשרות.

- **Secure** (מומלץ) - אימות חתום HMAC-SHA256 בצד השרת
- **Simple** - נתוני משתמש בצד הלקוח ללא חתימה
- **None** - תגובות אנונימיות, ללא שילוב התחברות ל-Moodle

#### Page Contexts

שולט היכן מופיעות תגובות:

- **Course pages** - תגובות בעמודי הקורס הראשיים
- **Module/activity pages** - תגובות בפעילויות ומשאבים בודדים
- **Both** - תגובות בכל סוגי העמודים

#### Commenting Style

בחר את חוויית התגובות. ראה [Commenting Styles](#moodle-commenting-styles) לצילומי מסך של כל מצב.

- **Comments** - ווידג'ט תגובות משורשרות סטנדרטי מתחת לתוכן העמוד
- **Collab Chat** - דיונים על בחירת טקסט בתוך העמוד עם מחווני נוכחות
- **Both** - תגובות וצ'אט שיתופי פעילים יחד

#### CDN URL

כתובת ה-CDN של FastComments. ברירת המחדל היא `https://cdn.fastcomments.com`. שנה זאת לכתובת ה-CDN של ה-EU אם הנתונים שלך מאוחסנים באזור ה-EU.