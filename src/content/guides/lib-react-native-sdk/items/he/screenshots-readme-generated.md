#### עיצוב: Erebus
![עיצוב: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### עיצוב: Default
![עיצוב: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### עורך WYSIWYG מקומי עם תמיכה בתמונות!
![עורך WYSIWYG מקומי עם תמיכה בתמונות](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### עורך טקסט עשיר

ספרייה זו משתמשת ב-[`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) לעריכת טקסט עשיר, שמספקת חווית עריכה WYSIWYG עוצמתית. אותו עורך מספק את היכולות ל-iOS, Android והווב (דרך `react-native-web`), ולכן המלחין מתנהג בעקביות בכל פלטפורמה עם יישום יחיד.

`react-native-enriched` דורש את React Native New Architecture (Fabric) על הנייטיב, ו-bundler שמפענח תנאי `exports` של חבילות (Metro עם package exports / RN 0.72+). התמיכה בווב היא כרגע ניסיונית.

### אפשרויות תצורה

מטרת ספרייה זו היא לתמוך בכל אפשרויות התצורה המוגדרות ב-[fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), בדיוק כמו במימוש של הווב.

### מושגי FastComments

המושגים העיקריים שכדאי להכיר כדי להתחיל הם `tenantId` ו-`urlId`. `tenantId` הוא מזהה חשבון FastComments.com שלך. `urlId` הוא המקום שאליו יקושרו שיחות התגובות. זה יכול להיות כתובת עמוד, או מזהה מוצר, מזהה מאמר וכו'.

### התראות משתמשים

FastComments תומכת בהתראות עבור [מצבים רבים](https://docs.fastcomments.com/guide-notifications.html). ההתראות ניתנות להגדרה, ניתן לוותר עליהן באופן גלובלי או ברמת התראה/תגובה, ותומכות במנויים ברמת העמוד כך שמשתמשים יכולים להירשם לשיחות תגובות של עמוד או מאמר מסוים.

למשל, אפשר להשתמש ב-Secure SSO לאימות המשתמש ואז לבצע פולינג התקופתי עבור התראות שלא נקראו ולדחוף אותן למשתמש.

ראו את [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) כדי לראות איך לקבל ולתרגם התראות משתמש שלא נקראו.

### דפדפן GIF

ברירת המחדל היא שאין אפשרות לבחירת תמונה או GIF מופעלת. ראו [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) להדרכה כיצד לתמוך בהעלאות תמונות ו-GIF. קיימת ספריה של דפדפן GIF שמשמיתה חיפושים ותמונות בספרייה זו, וכל מה שעליכם לעשות הוא להשתמש בה.

### ביצועים

אנא פתחו כרטיס עם דוגמה לשחזור, כולל המכשיר שבו השתמשתם, אם זיהיתם בעיות ביצועים. ביצועים הם שיקול מדרגה ראשונה בכל ספריות FastComments.