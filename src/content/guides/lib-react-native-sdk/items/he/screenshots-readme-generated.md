Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>הערות חיות</b><br/><img src="./demo-screenshots/light.png" width="260" alt="הערות חיות, ערכת נושא בהירה"/></td>
    <td align="center"><b>ערכת נושא כהה</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="הערות חיות, ערכת נושא כהה"/></td>
    <td align="center"><b>צ'אט חי</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="הגדרת צ'אט חי"/></td>
  </tr>
</table>

### עורך טקסט עשיר

ספרייה זו משתמשת ב-[`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) לעריכת טקסט עשיר, אשר מספקת חוויית עריכה WYSIWYG חזקה. אותו עורך מניע את iOS, Android והאינטרנט (באמצעות `react-native-web`), ולכן הקומפוזר מתנהג באופן עקבי על כל פלטפורמה עם יישום יחיד.

`react-native-enriched` דורש את ארכיטקטורת React Native החדשה (Fabric) במקורי (הברירה מ‑RN 0.76, אפשרות הפעלה ב‑RN 0.72‑0.75), ו‑bundler שמפענח תנאי `exports` של החבילה. SDK זה מפותח ונבדק מול RN 0.81 / React 19. אותו עורך עובד גם באינטרנט דרך `react-native-web`; בנייה האינטרנט של העורך המוגבר עדיין מסומנת כניסוי במקור.

### וידג'טים

ה‑SDK כולל שלושה וידג'טים, המשקפים את FastComments Android SDK:

- `FastCommentsLiveCommenting` – הערות חיות מונחות עם הצבעות, תגובות, דפדוף, אזכורים, הודעות, ועדכונים בזמן אמת.
- `FastCommentsLiveChat` – הגדרת צ'אט על אותו מנוע: הודעות כרונולוגיות עם חדשות בתחתית, הקומפוזר מתחת לרשימה, מסגרת כותרת חיה (נקודת חיבור + מספר משתמשים), היסטוריה אינסופית נטענת בגלילה למעלה, גלילה אוטומטית להודעות חדשות, ללא הצבעות או שרשור תגובות. כל הגדרה ניתנת לשינוי דרך `config`.
- `FastCommentsFeed` – פיד חברתי עם קומפוזר פוסט, מדיה, תגובות, מעקב, ובאנרים של פוסטים חדשים חיה.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### התאמת נושא

המראה ברירת המחדל נוצר מקבוצת אסימוני עיצוב סמנטיים (`FastCommentsTheme`): צבעים, ריווח, רדיוס, גדלי גופנים, משקלי גופנים, וגודלי אווטרים. העבירו דריסת אסימונים חלקית (Typed `FastCommentsThemeOverrides`) דרך הפרופ `theme` על כל וידג'ט והעץ הסגנוני כולו יעוצב מחדש באופן עקבי:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

מוד חושך הוא סט אסימונים אחד בלבד משם:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

ה‑prop `styles` עדיין מקבל עץ `IFastCommentsStyles` גולמי לשליטה מדויקת. כאשר `theme` ו‑`styles` מסופקים יחד, הסגנונות המפורשים מנצחים על עץ המוט; כאשר רק `styles` מסופק, הוא מחליף לחלוטין את ברירות המחדל (ההתנהגות המקורית, כך ששילובים ועורוות קיימים אינם מושפעים). `setupDarkModeSkin` הוסרה לטובת הפרופ `theme`.

### אפשרויות תצורה

ספרייה זו שואפת לתמוך בכל אפשרויות התצורה המוגדרות ב-[fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), בדיוק כמו מימוש האינטרנט.

בנוסף לכל אלה, React Native מוסיף כמה אפשרויות ספציפיות ל‑SDK דרך `FastCommentsRNConfig`:

- `hideTopBar` – הסתר את שורת העליונה של משתמש מחובר / פעמון ההודעה שמוצגת מעל הקומפוזר.
- `usePressToEdit` – הקשה והחזקת תגובה לפתיחת תפריט שלה.
- `disableDownVoting` – הסתר כפתורי הצבעה שלילית.
- `renderCommentInline` – הצג מידע של המגיב בתוך אותו בלוק HTML של תוכן התגובה.
- `renderLikesToRight` – הזז את אזור ההצבעה/אהבה לימין ההערה במקום מתחתיה.
- `renderDateBelowComment` – הצג את התאריך מתחת להערה.
- `showLiveStatus` – הצג את שורת הכותרת בסגנון צ'אט "חי" + מספר משתמשים מעל ההערות.
- `useInlineSubmitButton` – הצג את כפתור השליחה כאייקון בתוך הקומפוזר.
- `countAboveToggle` – עם `useShowCommentsToggle`, כמה תגובות יוצגו מעל המתג "הצג תגובות".
- `preserveFeedScrollPosition` – `FastCommentsFeed` שומרת את מיקום הגלילה שלה בין ניתוק/חיבור מחדש (ברירת מחדל true).

### מושגי FastComments

המושגים המרכזיים שעליכם לדעת כדי להתחיל הם `tenantId` ו‑`urlId`. `tenantId` הוא מזהה החשבון שלכם ב‑FastComments.com. `urlId` הוא המקום שבו חוטי ההערות יהיו קשורים. זה יכול להיות URL של דף, מזהה מוצר, מזהה מאמר, וכדומה.

### לוקאליזציה

כל הטקסט שמופיע למשתמשים בוידג'טים (תגיות כפתורים, מצייני מקום, מצבי ריק, תאריכים יחסיים כמו "לפני 5 דקות", הודעות שגיאה, ועוד) הוא **מונע על ידי השרת**. הרכיבים אינם מכילים מחרוזות קבועות באנגלית; הם מציגים את התרגומים ש‑FastComments מספק עבור השפה המבוקשת.

כדי לבקש שפה, הגדירו `locale` בתצורה שלכם:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, וכו'.
};
```

כאשר `locale` אינו מוגדר, FastComments מציג את שפת ברירת המחדל של השוכר.

**עריכת הטקסט:** תרגומים מנוהלים בלוח הבקרה של FastComments, ולא ב‑SDK הזה. כדי לשנות ניסוחים, יש לעדכן את העותק המוצע, או להוסיף שפה, על‑ידי עריכת ההתרגומים לחשבון שלכם בלוח הבקרה – השינוי ייתפס אוטומטית על‑ידי הוידג'טים ללא צורך בפריסת אפליקציה. ה‑SDK אינו כולל גיבוי אנגלית, ולכן כל מפתח שמשאירים ריק בלוח הבקרה מוצג כריק; שמרו את המפתחות מאוכלסים לכל שפה שאתם תומכים בה.

### התראות משתמש

FastComments תומך בהתראות עבור [תרחישים רבים](https://docs.fastcomments.com/guide-notifications.html). ההתראות ניתנות להגדרה, ניתן לבטל אותן גלובלית או ברמת התראה/הערה, ותומכות במנויים ברמת הדף כך שמשתמשים יכולים להירשם לחוטי שיחה של דף או מאמר ספציפי.

לדוגמה, ניתן להשתמש ב‑Secure SSO כדי לאמת את המשתמש ואז לתזמן משאלות תקופתיות להתראות שלא נקראו ולדחוף אותן למשתמש.

ראו את [הדוגמה AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) לקבלת מידע איך לקבל ולתרגם התראות משתמש שלא נקראו.

### דפדפן Gif

כברירת מחדל, לא מופעלת בחירת תמונה או gif. ראו את [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) לקבלת מידע איך לתמוך בטעינת תמונות ו‑gif. יש דפדפן Gif שמאנונימז את החיפושים והתמונות המסופקות בספרייה זו, פשוט השתמשו בו.

### ביצועים

אנא פתחו פנייה עם דוגמה לשחזור, כולל המכשיר שבו השתמשתם, אם אתם מזהים בעיות ביצועים. ביצועים הם נושא מרכזי בכל ספריות FastComments.