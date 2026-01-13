---
#### עיצוב: Erebus
![עיצוב: Erebus](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-erebus.PNG)
#### עיצוב: Default
![עיצוב: Default](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-default.PNG)
#### עורך WYSIWYG מקומי עם תמיכה בתמונות!
![עורך WYSIWYG מקומי עם תמיכה בתמונות](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/native-wysiwyg.PNG)

### עורך טקסט עשיר

ספרייה זו משתמשת בעורך 10tap עבור פונקציונליות עריכת טקסט עשיר, שמספק חוויית עריכה WYSIWYG עוצמתית.

### אפשרויות תצורה

ספרייה זו שואפת לתמוך בכל אפשרויות התצורה המוגדרות ב-[fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), בדיוק כמו במימוש עבור הווב.

### מושגים ב-FastComments

המושגים העיקריים שכדאי להכיר כדי להתחיל הם `tenantId` ו-`urlId`. `tenantId` הוא מזהה החשבון שלך ב-FastComments.com. `urlId` הוא הישות אליה יהיו מקושרות שרשורי התגובות. זה יכול להיות כתובת URL של עמוד, מזהה מוצר, מזהה מאמר וכו'.

### הודעות למשתמש

FastComments תומכת בהודעות עבור [מצבים רבים](https://docs.fastcomments.com/guide-notifications.html). ההודעות ניתנות לתצורה, ניתן לבטל רישום מהן באופן גלובלי או ברמת הודעה/תגובה, ותומכות במנויים ברמת עמוד כך שמשתמשים יכולים להירשם לשרשורי תגובות של עמוד או מאמר ספציפי.

לדוגמה, ניתן להשתמש ב-Secure SSO לאימות המשתמש ואז לבצע סריקה תקופתית עבור הודעות שלא נקראו ולדחוף אותן למשתמש.

ראה את הדוגמה AppNotificationSecureSSO ב-[example/src/AppNotificationsSecureSSO.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) כדי לראות כיצד לקבל ולתרגם הודעות משתמש שלא נקראו.

### דפדפן GIF

כברירת מחדל, אין תמיכה בבחירת תמונה או גיף. ראה את [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) כדי ללמוד כיצד לתמוך בהעלאות תמונה וגיף. קיימת ספרייה לגלישה בגיפים שמשתקת זהות בחיפושים ובתמונות המסופקות בספרייה זו — פשוט יש להשתמש בה.

### ביצועים

אנא פתח כרטיס עם דוגמה לשחזור, כולל המכשיר שבו השתמשת, אם אתה מזהה בעיות ביצועים. ביצועים הם נושא בעל חשיבות עליונה בכל ספריות FastComments.
---