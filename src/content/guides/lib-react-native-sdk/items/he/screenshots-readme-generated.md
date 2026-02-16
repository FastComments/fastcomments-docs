#### מראה: Erebus
![מראה: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### מראה: Default
![מראה: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### עורך WYSIWYG מקומי עם תמיכה בתמונות!
![עורך WYSIWYG מקומי עם תמיכה בתמונות](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### עורך טקסט עשיר

ספרייה זו משתמשת בעורך 10tap עבור פונקציונליות עריכת טקסט עשיר, המעניק חוויית עריכה WYSIWYG עוצמתית.

### אפשרויות תצורה

ספרייה זו שואפת לתמוך בכל אפשרויות התצורה המוגדרות ב-[fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), בדיוק כמו המימוש לווב.

### מושגי FastComments

המושגים העיקריים שיש להכיר כדי להתחיל הם `tenantId` ו-`urlId`. `tenantId` הוא מזהה חשבון FastComments.com שלך. `urlId` הוא המקום שאליו יקושרו שרשורי התגובות. זה יכול להיות כתובת דף, או מזהה מוצר, מזהה מאמר, וכו'.

### התראות משתמש

FastComments תומכת בהתראות עבור [תרחישים רבים](https://docs.fastcomments.com/guide-notifications.html). התראות ניתנות לקונפיגורציה, ניתן לבטל את ההרשמה אליהן ברמה גלובלית או ברמת התראה/תגובה, ותומכות במנויים ברמת דף כך שמשתמשים יכולים להירשם לשרשורים של דף או מאמר ספציפי.

לדוגמה, ניתן להשתמש ב-Secure SSO לאימות המשתמש ואז לבצע בדיקות תקופתיות להודעות שלא נקראו ולדחוף אותן למשתמש.

ראה את [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) כדי ללמוד כיצד לקבל ולהציג התראות משתמש שלא נקראו.

### דפדפן GIF

ברירת המחדל היא שאין בחירת תמונה או GIF מופעלת. ראה [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) כדי ללמוד איך לתמוך בהעלאות תמונה ו-GIF. ישנו דפדפן GIF שמאנונם חיפושים ותמונות שמסופקות בספרייה זו — כל מה שעליך לעשות הוא להשתמש בו.

### ביצועים

בבקשה פתח כרטיס עם דוגמה לשחזור, כולל המכשיר בו השתמשת, אם זיהית בעיות בביצועים. ביצועים הם מרכיב בכיר בכל ספריות FastComments.