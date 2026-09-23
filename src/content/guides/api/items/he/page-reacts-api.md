Page Reacts מאפשר למשתמשים שלך לאהוב דף, או להגיב אליו עם סט תמונות תגובה משלך. הווידג'ט [Page Reacts widget](/guide-page-reacts.html) והווידג'ט Floating Likes נבנים על בסיס קצות האינטרנט האלה, ואתה יכול לקרוא להם בעצמך כדי לבנות כפתור לייק משלך.

בשונה משאר המדריך, קצות האינטרנט של Page Reacts הם ציבוריים. הם נקראים מדפדפני המשתמשים שלך, אינם דורשים מפתח API, ולא צורכים קרדיטים של API. כל תגובה שייכת למשתמש שמבצע את הבקשה, ולכן משתמש יכול להוסיף או להסיר רק את שלו.

קיימים שני סטים של קצות אינטרנט:

- `/page-reacts/v1/likes/:tenantId` - "לייק" יחיד לכל משתמש לכל דף. השתמש בזה עבור כפתור לייק.
- `/page-reacts/v2/:tenantId` - תגובות מרובות לכל דף, כל אחת מזוהה על ידי `id` קצר שאתה בוחר (לדוגמה `heart` או `laugh`).

שניהם זמינים גם ב‑SDK שלנו כחלק מ‑`PublicApi`, לדוגמה `getV1PageLikes`, `createV1PageReact`, ו‑`deleteV1PageReact` ב‑[JavaScript SDK](/guide-sdk-javascript.html).

### זיהוי המשתמש

תגובות מקושרות למשתמש שמבצע את הבקשה:

- **משתמשי SSO:** העבר את פרמטר השאילתה `sso`, המוגדר ל‑JSON מקודד ב‑URI של אותו אובייקט SSO שאתה מספק לווידג'ט התגובות. ראה [SSO](/guide-customizations-and-configuration.html#sso).
- **משתמשים אנונימיים:** כאשר אין פרמטר `sso` ואין כניסה ל‑FastComments, השרת מקצה לדפדפן מזהה אנונימי שנשמר בעוגיית הסשן של FastComments. שלח בקשות עם `credentials: 'include'` כדי שהעוגייה תישמר בין הבקשות. דפדפנים החוסמים עוגיות של צד שלישי לא ישמרו את המזהה האנונימי, ולכן השתמש ב‑SSO כאשר יש צורך לזהות כל משתמש באופן אמין.

### ה‑urlId

`urlId` מזהה את הדף, בדיוק כפי שהוא עושה עבור תגובות. השתמש באותו `urlId` שאתה מספק לווידג'ט התגובות כך שהלייקים והתגובות ייספרו באותו דף. זכור לקודד אותו ב‑URI.

[inline-code-attrs-start title = 'דוגמת כפתור לייק'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Optional, for SSO users. The same object you give the comment widget's "sso" option.
const sso = null;

function likesUrl() {
    let url = `https://fastcomments.com/page-reacts/v1/likes/${tenantId}?urlId=${encodeURIComponent(urlId)}`;
    if (sso) {
        url += '&sso=' + encodeURIComponent(JSON.stringify(sso));
    }
    return url;
}

async function getLikes() {
    const response = await fetch(likesUrl(), {credentials: 'include'});
    return response.json(); // {status, likeCount, didLike, commentCount, urlIdWS}
}

async function like() {
    await fetch(likesUrl(), {method: 'POST', credentials: 'include'});
}

async function unlike() {
    await fetch(likesUrl(), {method: 'DELETE', credentials: 'include'});
}
[inline-code-end]