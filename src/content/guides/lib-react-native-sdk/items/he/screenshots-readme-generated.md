מערכת תגובות חיה בעלת שיח שרשתי עם אווטרים, תגובות מקוננות, הצבעות, ומהמחבר המובנה לעריכת טקסט עשיר, בנוסף ערכת נושא כהה ותצורת צ'אט חיה (מוצגת כאן באמצעות `react-native-web`):

<table>
  <tr>
    <td align="center"><b>תגובות חיות</b><br/><img src="./demo-screenshots/light.png" width="260" alt="תגובות חיות, ערכת נושא בהירה"/></td>
    <td align="center"><b>ערכת נושא כהה</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="תגובות חיות, ערכת נושא כהה"/></td>
    <td align="center"><b>צ'אט חי</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="תצורת צ'אט חיה"/></td>
  </tr>
</table>

### עורך טקסט עשיר

הספרייה משתמשת ב-[`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) לעריכת טקסט עשיר, שמספק חוויית עריכה WYSIWYG חזקה. אותו עורך מפעיל את iOS, Android וה-web (באמצעות `react-native-web`), כך שהמחבר מתנהג בצורה עקבית בכל פלטפורמה באמצעות מימוש יחיד.

`react-native-enriched` דורש את הארכיטקטורה החדשה של React Native (Fabric) על נייטיב (ברירת המחדל מאז RN 0.76, אופציונלי ב-RN 0.72-0.75), ובאנדלר שמסוגל לפתור תנאי `exports` של חבילות. SDK זה מפותח ונבדק נגד RN 0.81 / React 19. אותו עורך פועל גם ב-web דרך `react-native-web`; בניית ה-web של העורך enriched עדיין מסומנת כניסיונית בריפוזיטורי המקור.

### וידג'טים

ה-SDK כולל שלושה וידג'טים, המשקפים את ה-SDK של FastComments לאנדרואיד:

- `FastCommentsLiveCommenting` - תגובות שרשתי עם הצבעות, תגובות מקוננות, פג'ינציה, אזכורים, התראות, ועדכונים חיים.
- `FastCommentsLiveChat` - תצורת צ'אט המבוססת על אותו מנוע: הודעות כרונולוגיות כשהחדשות בתחתית, המחבר מתחת לרשימה, רצועת כותרת חיה (נקודת חיבור + ספירת משתמשים), היסטוריה אינסופית הטעונה על ידי גלילה מעלה, גלילה אוטומטית להודעות חדשות, ללא הצבעות או חוטי תגובות. כל תצורה ניתנת לעקיפה דרך `config`.
- `FastCommentsFeed` - פיד חברתי עם מחבר פוסטים, מדיה, ריאקציות, מעקבים, ושלטי פוסטים חדשים בזמן אמת.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### ערכת עיצוב

המראה ברירת המחדל נוצר מהגדרה של טוקנים סמנטיים לעיצוב (`FastCommentsTheme`): צבעים, ריווח, רדיוס, גדלי גופנים, עובי גופנים, וגדלי אווטרים. העבר סט חלקי של הטוקנים (הקלסי `FastCommentsThemeOverrides`) דרך הפרופ `theme` על כל וידג'ט וכל עץ הסגנון יתעדכן באופן עקבי:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

מצב כהה זמין על ידי החלפת סט הטוקנים:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

הפרופ `styles` עדיין מקבל עץ `IFastCommentsStyles` גולמי לשליטה מדויקת. כאשר גם `theme` וגם `styles` מסופקים, הסגנונות המפורשים גוברים על עץ הנושא; כאשר נמסר רק `styles`, הוא מחליף לחלוטין את ברירות המחדל (התנהגות מקורית, כך שישויות ושכבות קיימות אינן מושפעות). `setupDarkModeSkin` במצב מיושן לטובת הפרופ `theme`.

### אפשרויות תצורה

ספרייה זו שואפת לתמוך בכל אפשרויות התצורה המוגדרות ב-[fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), בדומה למימוש ה-web.

בנוסף לאלה, React Native מוסיף מספר אפשרויות ספציפיות ל-SDK דרך `FastCommentsRNConfig`:

- `hideTopBar` - להסתיר את רצועת המשתמש המחובר / פעמון ההתראות שמוצגת מעל המחבר.
- `usePressToEdit` - לחיצה והחזקת תגובה לפתיחת התפריט שלה.
- `disableDownVoting` - להסתיר את כפתורי ההצבעה נגד.
- `renderCommentInline` - להציג את פרטי המגיב בתוך אותו בלוק HTML כמו תוכן התגובה.
- `renderLikesToRight` - להעביר את אזור ההצבעות/לייקים לימין התגובה במקום מתחתיה.
- `renderDateBelowComment` - להציג את התאריך מתחת לתגובה.
- `showLiveStatus` - להציג את רצועת הכותרת בסגנון צ'אט עם "Live" + ספירת משתמשים מעל התגובות.
- `useInlineSubmitButton` - להציג את כפתור השליחה כאייקון בתוך המחבר.
- `countAboveToggle` - עם `useShowCommentsToggle`, כמה תגובות יופיעו מעל מתג "הצג תגובות".
- `preserveFeedScrollPosition` - `FastCommentsFeed` זוכר את הזזת הגלילה שלו בין פירוק/הרכבה מחדש (ברירת המחדל true).

### מושגי FastComments

המושגים העיקריים שכדאי להכיר כדי להתחיל הם `tenantId` ו-`urlId`. `tenantId` הוא מזהה החשבון שלך ב-FastComments.com. `urlId` הוא אליו יקושרו חוטי התגובות. זה יכול להיות כתובת דף, או מזהה מוצר, מזהה מאמר, וכו'.

### התראות משתמש

FastComments תומך בהתראות עבור [תסריטים רבים](https://docs.fastcomments.com/guide-notifications.html). התראות ניתנות לקונפיגורציה, ניתן להסיר את ההסכמה להן באופן גלובלי או ברמת התראה/תגובה, ותומך במנויים ברמת הדף כך שמשתמשים יכולים להירשם לחוטי תגובות של דף או מאמר ספציפיים.

לדוגמה, ניתן להשתמש ב-SSO מאובטח לאימות המשתמש ואז לבדוק באופן תקופתי הודעות שלא נקראו ולדחוף אותן אל המשתמש.

ראה את [הדוגמה AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) כדי לראות כיצד לקבל ולתרגם התראות משתמש שלא נקראו.

### דפדפן GIF

ברירת המחדל היא שלא מופעלת בחירת תמונה או GIF. ראה [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) כדי ללמוד כיצד לתמוך בהעלאת תמונות ו-GIF. קיימת ספרייה לדפדוף GIF שמאנוננת חיפושים ותמונות המסופקות בספרייה זו — פשוט יש להשתמש בה.

### ביצועים

אנא פתחו כרטיס עם דוגמה לשחזור הבעיה, כולל המכשיר שבו היא נצפתה, אם אתם מזהים בעיות ביצועים. ביצועים הם עקרון חשוב בכל ספריות FastComments.