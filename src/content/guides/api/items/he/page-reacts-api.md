Page Reacts מאפשר למשתמשים שלך לאהוב דף, או להגיב אליו עם סט של תמונות תגובה משלך. ה-[Page Reacts widget](/guide-page-reacts.html) והווידג'ט Floating Likes נבנים על בסיס קצות שירות אלה, ואתה יכול לקרוא להם בעצמך כדי לבנות כפתור לייק משלך.

בשונה משאר המדריך, קצות השירות של Page Reacts הם ציבוריים. הם נקראים מדפדפני המשתמשים שלך, אינם דורשים מפתח API, ואינם צורכים קרדיטים של API. כל תגובה שייכת למשתמש שמבצע את הבקשה, ולכן משתמש יכול להוסיף או להסיר רק את שלו.

קיימים שני סטים של קצות שירות:

- `/page-reacts/v1/likes/:tenantId` – לייק יחיד לכל משתמש לכל דף. השתמשו בזה עבור כפתור לייק.
- `/page-reacts/v2/:tenantId` – תגובות מרובות לכל דף, כל אחת מזוהה על‑ידי `id` קצר שאתם בוחרים (לדוגמה `heart` או `laugh`).

שניהם זמינים גם ב‑SDK שלנו כחלק מ‑`PublicApi`, לדוגמה `getV1PageLikes`, `createV1PageReact` ו‑`deleteV1PageReact` ב‑[JavaScript SDK](/guide-sdk-javascript.html).

### הצגת מי הגיב

העבירו `includeUserInfo=public` לקצות השירות `GET` כדי לקבל גם את המשתמשים שאהבו או הגיבו, עם שמות המשתמש והאוואטרים שלהם, newest first. משתמשים שהפכו את פעילות הפרופיל שלהם לפרטית אינם נכללים, וכן משתמשים אנונימיים. מוחזרים עד 100 תגובות.

משתמשי SSO נוצרו עם פעילות פרופיל פרטית אלא אם מטען ה‑SSO שלכם מגדיר `isProfileActivityPrivate` ל‑`false`, ולכן יש להגדיר זאת עבור המשתמשים שברצונכם שיופיעו ברשימות אלו.

בדפים הדורשים SSO, רק משתמשי SSO שיכולים לצפות בדף יכולים לרשום מי הגיב. אחרים מקבלים `403` עם הקוד `sso-required` או `page-forbidden`.

מהשרת שלכם, [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) ו‑[`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) מחזירים את אותן רשימות עם מפתח API, וכן מקבלים `includeUserInfo=include-private` כדי לכלול משתמשים עם פרופילים פרטיים.

### זיהוי המשתמש

תגובות משויכות למשתמש שמבצע את הבקשה:

- **משתמשי SSO:** העבירו את פרמטר השאילתה `sso`, המוגדר ל‑JSON המקודד ב‑URI של אותו אובייקט SSO שאתם נותנים ל‑widget התגובה. ראו [SSO](/guide-customizations-and-configuration.html#sso).
- **משתמשים אנונימיים:** כאשר אין פרמטר `sso` ואין התחברות FastComments, השרת מקצה לדפדפן מזהה אנונימי הנשמר בעוגיית הסשן של FastComments. שלחו בקשות עם `credentials: 'include'` כדי שהעוגייה תישמר בין הבקשות. דפדפנים החוסמים עוגיות צד שלישי לא ישמרו את המזהה האנונימי, ולכן השתמשו ב‑SSO כאשר יש צורך בזיהוי אמין של כל משתמש.

### ה‑urlId

`urlId` מזהה את הדף, בדיוק כפי שהוא עושה עבור תגובות. השתמשו באותו `urlId` שאתם נותנים ל‑widget התגובה כך שהלייקים והתגובות ייספרו באותו דף. זכרו לקודד אותו ב‑URI.

[inline-code-attrs-start title = 'דוגמת כפתור לייק'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// אופציונלי, למשתמשי SSO. אותו אובייקט שאתה מספק לאופציית "sso" של וידג'ט התגובה.
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

---