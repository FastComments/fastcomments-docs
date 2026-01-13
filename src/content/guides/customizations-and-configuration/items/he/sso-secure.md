[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO משתמש בהצפנת HMAC-SHA256 כמנגנון למימוש SSO. ראשית נסקור את הארכיטקטורה הכוללת, נספק דוגמאות ושלבים מפורטים.

יש גם תיעוד לגבי מעבר מספקים אחרים עם מנגנוני SSO דומים, וההבדלים.

The flow looks like this:

<div class="screenshot white-bg">
    <div class="title">Secure SSO Flow</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Secure SSO Diagram" />
</div>

מאחר ש-Secure SSO כולל פיתוח full-stack, דוגמאות קוד עובדות מלאות ב-Java/Spring, NodeJS/Express, ו-PHP וניתנות כעת <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">ב-GitHub</a>.

למרות שאנו משתמשים ב-ExpressJS בדוגמת ה-NodeJS וב-Spring בדוגמת ה-Java, אין צורך בספריות/פריימוורקים בריצות אלו כדי לממש את FastComments SSO - חבילות הקריפטו המובנות מספקות את הדרוש.

אין צורך לכתוב נקודות קצה חדשות עם FastComments SSO. פשוט הצפן את פרטי המשתמש באמצעות המפתח הסודי שלך והעבר את ה-payload לווידג'ט התגובות.

#### קבל את מפתח ה-API הסודי שלך

מפתח ה-API הסודי שלך ניתן לשליפה מ- <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">העמוד הזה</a>. ניתן גם למצוא דף זה על ידי כניסה ל-My Account, לחיצה על הריבוע API/SSO, ואז לחיצה על "Get API Secret Key".

#### פרמטרים של ווידג'ט התגובות

תיעוד API ברמה גבוהה עבור ווידג'ט התגובות ניתן למצוא <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">כאן</a>.

נפרט כעת בצורה מעמיקה יותר מה המשמעות של פרמטרים אלה.

ווידג'ט התגובות לוקח אובייקט קונפיגורציה - אתה כבר מעביר זאת אם אתה משתמש ב-FastComments כדי להעביר את מזהה הלקוח שלך (נקרא tenantId).

כדי לאפשר SSO, העבר אובייקט "sso" חדש, שחייב להכיל את הפרמטרים הבאים. הערכים אמורים להיווצר בצד השרת.

- userDataJSONBase64: The user's data in JSON format, which is then Base64 encoded.
- verificationHash: The HMAC-SHA256 hash created from UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Epoch timestamp, in **milliseconds**. Must not be in the future, or more than two days in the past.
- loginURL: A URL that the comment widget can show to log the user in.
- logoutURL: A URL that the comment widget can show to log the user out.
- loginCallback: When provided instead of the login URL, a function that the comment widget will invoke when clicking the login button.
- logoutCallback: When provided instead of the logout URL, a function that the comment widget will invoke when clicking the logout button.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### אובייקט המשתמש

The User object contains the following schema:
[inline-code-attrs-start title = 'אובייקט המשתמש'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** חובה. מקסימום 1k תווים. **/
    id: string;
    /** חובה. מקסימום 1k תווים. הערה: חייב להיות ייחודי. **/
    email: string;
    /** חובה. מקסימום 1k תווים. הערה: שם המשתמש לא יכול להיות אימייל. לא חייב להיות ייחודי. **/
    username: string;
    /** אופציונלי. מקסימום 3k תווים עבור כתובות URL. ברירת המחדל היא gravatar המבוסס על אימייל. תומך בתמונות מקודדות ב-64, במקרה כזה המגבלה היא 50k תווים. **/ 
    avatar?: string;
    /** אופציונלי. ברירת מחדל: false. **/
    optedInNotifications?: boolean;
    /** אופציונלי. ברירת מחדל: false. **/
    optedInSubscriptionNotifications?: boolean;
    /** אופציונלי. מקסימום 100 תווים. תווית זו תוצג לצד שמם. ברירת המחדל היא מנהל/מנחה כאשר רלוונטי. **/
    displayLabel?: string;
    /** אופציונלי. מקסימום 500 תווים. זה יוצג במקום שם המשתמש. **/
    displayName?: string;
    /** אופציונלי. מקסימום 2k תווים. שם המשתמש יקשר לכתובת זו. **/
    websiteUrl?: string;
    /** אופציונלי. עד 100 קבוצות לכל משתמש. מזהה קבוצה לא יכול להיות ארוך מ-50 תווים. **/
    groupIds?: string[];
    /** אופציונלי. מגדיר את המשתמש כמנהל. **/
    isAdmin?: boolean;
    /** אופציונלי. מגדיר את המשתמש כמנחה. **/
    isModerator?: boolean;
    /** אופציונלי, ברירת מחדל true. הגדר ל-false כדי לאפשר את לשונית "פעילות" בפרופיל המשתמש. **/
    isProfileActivityPrivate?: boolean;
    /** אופציונלי, ברירת מחדל false. הגדר ל-true כדי להשבית תגובות בפרופיל. **/
    isProfileCommentsPrivate?: boolean;
    /** אופציונלי, ברירת מחדל false. הגדר ל-true כדי להשבית שליחת הודעות ישירות למשתמש זה. **/
    isProfileDMDisabled?: boolean;
}
[inline-code-end]

#### Moderators and Administrators

For admins and moderators, pass the respective `isAdmin` or `isModerator` flags in the `SSOUser` object.

#### Notifications

To enable or disable notifications, set the value of `optedInNotifications` to `true` or `false` respectively. The first time the user loads the page with this value in the SSO payload, their notification settings will be updated.

Additionally, if you want users to receive notification emails for activity on pages they subscribed to (as opposed to just in-app notifications), then set `optedInSubscriptionNotifications` to `true`.

#### VIP Users & Special Labels

You can display a special label next to the user's name by using the optional "displayLabel" field.

#### Unauthenticated users

To represent an unauthenticated user, simply do not populate userDataJSONBase64, verificationHash, or timestamp. Provide a loginURL.

These users will not be able to comment, and instead will be presented with a login message (message, link, or button, depending on configuration).

#### Direct Examples for Serializing and Hashing User Data

More details as an examples <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">here</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">here</a> (java) and <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">here</a> (php).

We understand that any integration can be a complicated and painful process. Don't hesitate to reach out to your representative or use the <a href="https://fastcomments.com/auth/my-account/help" target="_blank">support page</a>.