[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO משתמש בהצפנת HMAC-SHA256 כמנגנון למימוש SSO. ראשית נסקור את הארכיטקטורה הכללית, נספק דוגמאות, ושלבים מפורטים.

קיימת גם תיעוד בנוגע להגירה מספקים אחרים עם מנגנוני SSO דומים, וההבדלים ביניהם.

הזרימה נראית כך:

<div class="screenshot white-bg">
    <div class="title">זרימת Secure SSO</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="דיאגרמת Secure SSO" />
</div>

מכיוון ש-Secure SSO כולל פיתוח full-stack, דוגמאות קוד עובדות מלאות ב-Java/Spring, NodeJS/Express ו-PHP רגיל זמינות כעת <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">ב-GitHub</a>.

למרות שאנו משתמשים ב-ExpressJS בדוגמת ה-NodeJS וב-Spring בדוגמת ה-Java, אין צורך בפריימוורקים/ספריות בסביבות ריצה אלו למימוש FastComments SSO - חבילות ה-crypto המובנות מספיקות.

אינכם צריכים לכתוב נקודות קצה (API endpoints) חדשות עם FastComments SSO. פשוט הצפינו את פרטי המשתמש באמצעות מפתח הסוד שלכם והעבירו את ה-payload לווידג'ט התגובות.

#### קבלת מפתח הסוד של ה-API

ניתן לאחזר את מפתח הסוד של ה-API מ<a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">עמוד זה</a>. ניתן למצוא עמוד זה גם על ידי כניסה ל-My Account, לחיצה על אריח ה-API/SSO, ולאחר מכן לחיצה על "Get API Secret Key".

#### פרמטרים של ווידג'ט התגובות

תיעוד API ברמה גבוהה עבור ווידג'ט התגובות זמין <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">כאן</a>.

בואו נצלול לפרטים נוספים לגבי משמעות הפרמטרים הללו.

ווידג'ט התגובות מקבל אובייקט קונפיגורציה - אתם כבר מעבירים אותו אם אתם משתמשים ב-FastComments להעברת מזהה הלקוח שלכם (הנקרא tenantId).

כדי להפעיל SSO, העבירו אובייקט "sso" חדש, שחייב לכלול את הפרמטרים הבאים. הערכים צריכים להיווצר בצד השרת.

- userDataJSONBase64: נתוני המשתמש בפורמט JSON, אשר מקודדים לאחר מכן ב-Base64.
- verificationHash: ה-hash של HMAC-SHA256 שנוצר מ-UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: חותמת זמן Epoch, ב**מילישניות**. אסור שתהיה בעתיד, או יותר מיומיים בעבר.
- loginURL: כתובת URL שווידג'ט התגובות יכול להציג להתחברות המשתמש.
- logoutURL: כתובת URL שווידג'ט התגובות יכול להציג להתנתקות המשתמש.
- loginCallback: כאשר מסופק במקום כתובת ההתחברות, פונקציה שווידג'ט התגובות יפעיל בלחיצה על כפתור ההתחברות.
- logoutCallback: כאשר מסופק במקום כתובת ההתנתקות, פונקציה שווידג'ט התגובות יפעיל בלחיצה על כפתור ההתנתקות.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'קוד צד הלקוח של Secure SSO'; isFunctional = false; code-example-end]

#### אובייקט המשתמש

אובייקט המשתמש מכיל את הסכמה הבאה:
[inline-code-attrs-start title = 'אובייקט המשתמש'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** חובה. מקסימום 1k תווים. **/
    id: string;
    /** חובה. מקסימום 1k תווים. הערה: חייב להיות ייחודי. **/
    email: string;
    /** חובה. מקסימום 1k תווים. הערה: שם המשתמש לא יכול להיות כתובת אימייל. לא חייב להיות ייחודי. **/
    username: string;
    /** אופציונלי. מקסימום 3k תווים עבור כתובות URL. ברירת המחדל היא מ-gravatar לפי האימייל. תומך בתמונות מקודדות 64, ובמקרה זה המגבלה היא 50k תווים. **/ 
    avatar?: string;
    /** אופציונלי. ברירת מחדל false. **/
    optedInNotifications?: boolean;
    /** אופציונלי. ברירת מחדל false. **/
    optedInSubscriptionNotifications?: boolean;
    /** אופציונלי. מקסימום 100 תווים. תווית זו תוצג ליד שם המשתמש. ברירת המחדל היא Administrator/Moderator כשרלוונטי. **/
    displayLabel?: string;
    /** אופציונלי. מקסימום 500 תווים. יוצג במקום שם המשתמש. **/
    displayName?: string;
    /** אופציונלי. מקסימום 2k תווים. שם המשתמש יקושר לכתובת זו. **/
    websiteUrl?: string;
    /** אופציונלי. עד 100 קבוצות למשתמש. מזהה קבוצה לא יכול להיות ארוך מ-50 תווים. **/
    groupIds?: string[];
    /** אופציונלי. מסמן את המשתמש כמנהל מערכת. **/
    isAdmin?: boolean;
    /** אופציונלי. מסמן את המשתמש כמנחה (moderator). **/
    isModerator?: boolean;
    /** אופציונלי, ברירת מחדל true. הגדירו ל-false כדי להפעיל את טאב ה"פעילות" בפרופיל המשתמש. **/
    isProfileActivityPrivate?: boolean;
    /** אופציונלי, ברירת מחדל false. הגדירו ל-true כדי להשבית תגובות פרופיל. **/
    isProfileCommentsPrivate?: boolean;
    /** אופציונלי, ברירת מחדל false. הגדירו ל-true כדי להשבית שליחת הודעות ישירות למשתמש זה. **/
    isProfileDMDisabled?: boolean;
    /** הגדרות אופציונליות עבור תגי משתמש. **/
    badgeConfig?: {
        /** מערך של מזהי תגים גלובליים להקצאה. מוגבל ל-30 תגים. הסדר נשמר. **/
        badgeIds: string[];
        /** מערך של מזהי תגים המוגבלים לעמוד הנוכחי (urlId). מוצגים רק בעמוד שהוקצה להם. **/
        pageBadgeIds?: string[];
        /** אם true, מחליף תגים מוצגים קיימים. גלובליים ותגי עמוד מוחלפים באופן עצמאי. **/
        override?: boolean;
        /** אם true, מעדכן את מאפייני התצוגה של התגים מקונפיגורציית הטננט. **/
        update?: boolean;
    };
}
[inline-code-end]

#### מנחים ומנהלי מערכת

עבור מנהלי מערכת ומנחים, העבירו את הדגלים `isAdmin` או `isModerator` המתאימים באובייקט `SSOUser`.

#### התראות

כדי להפעיל או לכבות התראות, הגדירו את הערך של `optedInNotifications` ל-`true` או `false` בהתאמה. בפעם הראשונה שהמשתמש יטען את העמוד עם ערך זה ב-payload של ה-SSO, הגדרות ההתראות שלו יעודכנו.

בנוסף, אם ברצונכם שמשתמשים יקבלו אימיילי התראות עבור פעילות בעמודים שנרשמו אליהם (בניגוד להתראות בתוך האפליקציה בלבד), הגדירו את `optedInSubscriptionNotifications` ל-`true`.

#### משתמשי VIP ותוויות מיוחדות

ניתן להציג תווית מיוחדת ליד שם המשתמש באמצעות השדה האופציונלי "displayLabel".

#### משתמשים לא מאומתים

כדי לייצג משתמש לא מאומת, פשוט אל תאכלסו את userDataJSONBase64, verificationHash או timestamp. ספקו loginURL.

משתמשים אלה לא יוכלו להגיב, ובמקום זאת יוצג להם הודעת התחברות (הודעה, קישור או כפתור, בהתאם לקונפיגורציה).

#### דוגמאות ישירות לסריאליזציה ו-hash של נתוני משתמש

פרטים נוספים כדוגמה <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/node-express/routes/index.js#L23" target="_blank">כאן</a> (js), <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/java-springboot/src/main/java/com/winricklabs/ssodemo/DemoController.java#L68" target="_blank">כאן</a> (java) ו-<a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">כאן</a> (php).

אנו מבינים שכל אינטגרציה יכולה להיות תהליך מורכב ומתיש. אל תהססו לפנות לנציג שלכם או להשתמש ב<a href="https://fastcomments.com/auth/my-account/help" target="_blank">עמוד התמיכה</a>.