תגובות חיות בשרשור עם אווטארים, תגובות מקוננות, הצבעות ומחבר הטקסט העשיר המובנה, ובנוסף ערכת נושא כהה והגדרה מוכנה של צ'אט חי (מוצג כאן ברינדור באמצעות `react-native-web`):

<table>
  <tr>
    <td align="center"><b>תגובות חיות</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="תגובות חיות, ערכת נושא בהירה"/></td>
    <td align="center"><b>ערכת נושא כהה</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="תגובות חיות, ערכת נושא כהה"/></td>
    <td align="center"><b>צ'אט חי</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="הגדרת צ'אט חי"/></td>
  </tr>
</table>

### עורך טקסט עשיר

הספרייה משתמשת ב-[`react-native-enriched`](https://github.com/software-mansion/react-native-enriched-html) לעריכת טקסט עשיר, המספק חוויית עריכת WYSIWYG חזקה. אותו עורך מניע את iOS, Android והאינטרנט (דרך `react-native-web`), כך שהמחבר מתנהג באופן עקבי בכל הפלטפורמות עם מימוש אחד.

`react-native-enriched` דורש את הארכיטקטורה החדשה של React Native (Fabric) בסביבה המקורית (ברירת המחדל מאז RN 0.76, הפעלה ידנית ב-RN 0.72-0.75), ו-bundler שתומך בתנאי `exports` של חבילות. ה-SDK מפותח ונבדק מול RN 0.81 / React 19. אותו עורך רץ גם באינטרנט דרך `react-native-web`; ה-build לאינטרנט של העורך המועשר עדיין מסומן כניסיוני במקור.

### ווידג'טים

ה-SDK כולל שלושה ווידג'טים, בהתאמה ל-FastComments Android SDK:

- `FastCommentsLiveCommenting` - תגובות בשרשור עם הצבעות, תגובות השבה, עימוד, אזכורים, התראות ועדכונים חיים.
- `FastCommentsLiveChat` - הגדרת צ'אט על אותו מנוע: הודעות כרונולוגיות עם חדשות בתחתית, המחבר מתחת לרשימה, פס כותרת חי (נקודת חיבור + מספר משתמשים), היסטוריה אינסופית הנטענת בגלילה למעלה, גלילה אוטומטית להודעות חדשות, ללא הצבעות או שרשור תגובות. ניתן לדרוס כל הגדרה באמצעות `config`.
- `FastCommentsFeed` - פיד חברתי עם מחבר פוסטים, מדיה, תגובות (reactions), מעקבים ובאנרים חיים של פוסטים חדשים.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### עיצוב (Theming)

מראה ברירת המחדל נוצר מקבוצת טוקנים סמנטיים לעיצוב (`FastCommentsTheme`): צבעים, מרווחים, רדיוס, גדלי גופן, משקלי גופן וגדלי אווטאר. העבירו עדכונים חלקיים לטוקנים (מסוג `FastCommentsThemeOverrides`) דרך ה-prop של `theme` בכל ווידג'ט ועץ הסגנון כולו יעוצב מחדש באופן עקבי:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

מצב כהה נמצא במרחק סט טוקנים אחד:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

ה-prop של `styles` עדיין מקבל עץ `IFastCommentsStyles` גולמי לשליטה מדויקת. כאשר `theme` ו-`styles` מסופקים שניהם, הסגנונות המפורשים גוברים על עץ העיצוב; כאשר רק `styles` מסופק, הוא מחליף לחלוטין את ברירות המחדל (ההתנהגות המקורית, כך שאינטגרציות וסקינים קיימים אינם מושפעים). `setupDarkModeSkin` אינו בשימוש עוד לטובת ה-prop של `theme`.

### אפשרויות תצורה

הספרייה שואפת לתמוך בכל אפשרויות התצורה המוגדרות ב-[fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), בדיוק כמו מימוש האינטרנט.

בנוסף לאלה, React Native מוסיף כמה אפשרויות ייחודיות ל-SDK דרך `FastCommentsRNConfig`:

- `hideTopBar` - הסתרת הפס של המשתמש המחובר / פעמון ההתראות המוצג מעל המחבר.
- `usePressToEdit` - לחיצה ארוכה על תגובה פותחת את התפריט שלה.
- `disableDownVoting` - הסתרת כפתורי הצבעה שלילית.
- `renderCommentInline` - הצגת פרטי המגיב בתוך אותו בלוק HTML כתוכן התגובה.
- `renderLikesToRight` - העברת אזור ההצבעה/לייקים לימין התגובה במקום מתחתיה.
- `renderDateBelowComment` - הצגת התאריך מתחת לתגובה.
- `showLiveStatus` - הצגת פס כותרת בסגנון צ'אט "Live" + מספר משתמשים מעל התגובות.
- `useInlineSubmitButton` - הצגת כפתור השליחה כאייקון בתוך המחבר.
- `countAboveToggle` - עם `useShowCommentsToggle`, כמה תגובות יוצגו מעל מתג "Show Comments".
- `preserveFeedScrollPosition` - `FastCommentsFeed` זוכר את מצב הגלילה שלו בין פריקה לטעינה מחדש (ברירת מחדל true).

### מושגי FastComments

המושגים העיקריים שיש להכיר כדי להתחיל הם `tenantId` ו-`urlId`. `tenantId` הוא מזהה חשבון FastComments.com שלכם. `urlId` הוא המקום אליו יהיו קשורים שרשורי התגובות. זה יכול להיות כתובת URL של עמוד, או מזהה מוצר, מזהה מאמר וכו'.

### לוקליזציה

כל הטקסטים המוצגים למשתמש בווידג'טים אלה (תוויות כפתורים, placeholders, מצבים ריקים, תאריכים יחסיים כמו "לפני 5 דקות", הודעות שגיאה וכו') מנוהלים **על ידי השרת**. הרכיבים אינם מקודדים מחרוזות באנגלית; הם מציגים את התרגומים ש-FastComments מספקת עבור ה-locale המבוקש.

כדי לבקש locale, הגדירו `locale` בתצורה שלכם:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

כאשר לא מוגדר `locale`, FastComments מספקת את שפת ברירת המחדל של החשבון.

**עריכת הטקסטים:** התרגומים מנוהלים בדשבורד FastComments שלכם, לא ב-SDK הזה. כדי לשנות ניסוח, לדרוס את טקסט ברירת המחדל או להוסיף שפה, ערכו את התרגומים עבור החשבון שלכם בדשבורד - השינוי נקלט בווידג'טים אוטומטית ללא צורך בשחרור גרסה של האפליקציה. ה-SDK אינו כולל fallbacks באנגלית, כך שכל מפתח שתנקו בדשבורד יוצג ריק; שמרו על מילוי המפתחות עבור כל locale שאתם תומכים בו.

### התראות משתמש

FastComments תומכת בהתראות עבור [תרחישים רבים](https://docs.fastcomments.com/guide-notifications.html). ההתראות ניתנות להגדרה, ניתן לבטל הצטרפות ברמה גלובלית או ברמת התראה/תגובה, וקיימת תמיכה בהרשמות ברמת עמוד כך שמשתמשים יכולים להירשם לשרשורים של עמוד או מאמר ספציפי.

לדוגמה, ניתן להשתמש ב-SSO מאובטח לאימות המשתמש ואז לבצע polling תקופתי להתראות שלא נקראו ולהציג אותן למשתמש.

ראו [את הדוגמה AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) לאופן קבלה ותרגום של התראות משתמש שלא נקראו.

### דפדפן GIF

כברירת מחדל, בחירת תמונות או GIF אינה מופעלת. ראו [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) לאופן תמיכה בהעלאת תמונות ו-GIF. קיים דפדפן GIF שמאנונימז חיפושים ותמונות המסופק בספרייה הזו, כל שעליכם לעשות הוא להשתמש בו.

### ביצועים

אם אתם מזהים בעיות ביצועים, אנא פתחו כרטיס עם דוגמה לשחזור, כולל המכשיר בו נעשה שימוש. ביצועים הם עדיפות ראשונה בכל הספריות של FastComments.