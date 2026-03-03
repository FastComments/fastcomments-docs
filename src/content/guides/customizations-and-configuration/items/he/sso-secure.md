[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO משתמש בהצפנת HMAC-SHA256 כמנגנון ליישום SSO. תחילה נסביר את הארכיטקטורה הכוללת, נספק דוגמאות ושלבים מפורטים.

קיימת גם תיעוד לגבי מעבר מספקים אחרים עם מנגנוני SSO דומים, וההבדלים ביניהם.

הזרימה נראית כך:

<div class="screenshot white-bg">
    <div class="title">זרימת SSO מאובטח</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="תרשים SSO מאובטח" />
</div>

מכיוון ש-Secure SSO מערב פיתוח מלא-סטאק, דוגמאות קוד עובדות מלאות ב-Java/Spring, NodeJS/Express, ו-PHP פשוט נמצאות כרגע <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">ב-GitHub</a>.

למרות שבדוגמת ה-NodeJS אנו משתמשים ב-ExpressJS ובדוגמת ה-Java ב-Spring, אין צורך בספריות/פריימוורקים בריצות אלו כדי לממש את FastComments SSO — חבילות הקריפטו המובנות מספיקות.

אין צורך לכתוב נקודות קצה API חדשות עם FastComments SSO. פשוט הצפן את המידע של המשתמש בעזרת המפתח הסודי שלך והעבר את המטען ל-comment widget.

#### Get Your API Secret Key

מפתח ה-API הסודי שלך ניתן להשגה בדף זה: <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">this page</a>. ניתן למצוא דף זה גם על ידי מעבר ל-My Account, לחיצה על ריבוע ה-API/SSO, ואז לחיצה על "Get API Secret Key".

#### Comment Widget Parameters

תיעוד API ברמה גבוהה עבור ה-comment widget ניתן למצוא <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">כאן</a>.

נפרט כעת מה משמעות הפרמטרים הללו.

הווידג'ט של התגובות מקבל אובייקט תצורה — אתה כבר מעביר זאת אם אתה משתמש ב-FastComments כדי להעביר את מזהה הלקוח שלך (נקרא tenantId).

כדי לאפשר SSO, העבר אובייקט חדש בשם "sso", אשר חייב להכיל את הפרמטרים הבאים. הערכים צריכים להיווצר בצד השרת.

- userDataJSONBase64: המידע של המשתמש בפורמט JSON, שאחר כך מקודד ב-Base64.
- verificationHash: ה-HMAC-SHA256 שנוצר מ-UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: חותם זמן אפוק, במילישניות (**milliseconds**). אסור שיהיה בעתיד, או יותר משני ימים בעבר.
- loginURL: כתובת URL שהווידג'ט יכול להציג כדי להכניס את המשתמש.
- logoutURL: כתובת URL שהווידג'ט יכול להציג כדי לבצע התנתקות של המשתמש.
- loginCallback: כאשר מסופק במקום כתובת ה-login, פונקציה שהווידג'ט יזמן בלחיצה על כפתור ההתחברות.
- logoutCallback: כאשר מסופק במקום כתובת ה-logout, פונקציה שהווידג'ט יזמן בלחיצה על כפתור ההתנתקות.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### אובייקט המשתמש

The User Object
[inline-code-attrs-start title = 'אובייקט המשתמש'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** דרוש. מקסימום 1k תווים. **/
    id: string;
    /** דרוש. מקסימום 1k תווים. הערה: חייב להיות ייחודי. **/
    email: string;
    /** דרוש. מקסימום 1k תווים. הערה: שם המשתמש אינו יכול להיות אימייל. לא חייב להיות ייחודי. **/
    username: string;
    /** אופציונלי. מקסימום 3k תווים עבור כתובות URL. ברירת המחדל נלקחת מ-gravatar בהתבסס על המייל. תומך בתמונות מקודדות ב-Base64, שבמקרה זה המגבלה היא 50k תווים. **/ 
    avatar?: string;
    /** אופציונלי. ברירת מחדל false. **/
    optedInNotifications?: boolean;
    /** אופציונלי. ברירת מחדל false. **/
    optedInSubscriptionNotifications?: boolean;
    /** אופציונלי. מקסימום 100 תווים. תווית זו תוצג ליד שמם. ברירת מחדל היא מנהל/מנחה כאשר חל הדבר. **/
    displayLabel?: string;
    /** אופציונלי. מקסימום 500 תווים. זה יוצג במקום שם המשתמש. **/
    displayName?: string;
    /** אופציונלי. מקסימום 2k תווים. שם המשתמש ייקשר לכאן. **/
    websiteUrl?: string;
    /** אופציונלי. עד 100 קבוצות למשתמש. מזהה קבוצה לא יכול להיות ארוך מ-50 תווים. **/
    groupIds?: string[];
    /** אופציונלי. מסמן את המשתמש כמנהל. **/
    isAdmin?: boolean;
    /** אופציונלי. מסמן את המשתמש כמנחה. **/
    isModerator?: boolean;
    /** אופציונלי, ברירת מחדל true. הגדר ל-false כדי לאפשר את לשונית "activity" בפרופיל המשתמש. **/
    isProfileActivityPrivate?: boolean;
    /** אופציונלי, ברירת מחדל false. הגדר ל-true כדי להשבית תגובות בפרופיל. **/
    isProfileCommentsPrivate?: boolean;
    /** אופציונלי, ברירת מחדל false. הגדר ל-true כדי להשבית הודעות פרטיות למשתמש זה. **/
    isProfileDMDisabled?: boolean;
    /** קונפיגורציה אופציונלית של תגי משתמש. **/
    badgeConfig?: {
        /** מערך של מזהי תגי גלובל שיוקצו. מוגבל ל-30 תגיות. הסדר נשמר. **/
        badgeIds: string[];
        /** מערך של מזהי תגיות המוגדרים לעמוד הנוכחי (urlId). מוצגים רק בעמוד שמוקצה להם. **/
        pageBadgeIds?: string[];
        /** אם true, מחליף תגיות מוצגות קיימות. גלובליות ועמוד-ממוקדות מוחלפות באופן עצמאי. **/
        override?: boolean;
        /** אם true, מעדכן את תכונות התצוגה של תגיות מתוך קונפיגורציית ה-tenant. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderators and Administrators

עבור מנהלים ומנחים, העבר את הדגלים המתאימים `isAdmin` או `isModerator` באובייקט `SSOUser`.

#### Notifications

כדי להפעיל או לנטרל התראות, הגדר את הערך של `optedInNotifications` ל-`true` או `false` בהתאמה. בפעם הראשונה שהמשתמש טוען את הדף עם ערך זה ב-payload של SSO, הגדרות ההתראות שלו יעודכנו.

בנוסף, אם ברצונך שמשתמשים יקבלו הודעות דוא"ל על פעילות בעמודים שהם נרשמו אליהם (בניגוד רק להתראות בתוך האפליקציה), הגדר את `optedInSubscriptionNotifications` ל-`true`.

#### VIP Users & Special Labels

ניתן להציג תווית מיוחדת ליד שמו של המשתמש באמצעות השדה האופציונלי "displayLabel".

#### Unauthenticated users

כדי לייצג משתמש לא מאומת, פשוט אל תמלא את userDataJSONBase64, verificationHash, או timestamp. ספק כתובת loginURL.

משתמשים אלה לא יוכלו להגיב, ובמקומם יוצג להם הודעת התחברות (הודעה, קישור, או כפתור, בהתאם לקונפיגורציה).

#### Direct Examples for Serializing and Hashing User Data

פרטים נוספים ודוגמאות נמצאים <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">כאן</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">כאן</a> (java) ו-<a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">כאן</a> (php).

אנו מבינים שכל אינטגרציה יכולה להיות תהליך מסובך וכואב. אל תהססו ליצור קשר עם הנציג שלכם או להשתמש בדף התמיכה <a href="https://fastcomments.com/auth/my-account/help" target="_blank">support page</a>.