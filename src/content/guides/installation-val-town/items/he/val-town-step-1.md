הווידג'ט הוא תג script ואלמנט מכולה, ולכן הוא נופל לתוך מה שה‑val שלך כבר מציג. דוגמה זו משתמשת ב‑Hono JSX, שהיא מה שה‑templates של HTTP ב‑Val Town משתמשים.

[inline-code-attrs-start title = 'ווידג\'ט תגובה ב‑HTTP val'; type='javascript' inline-code-attrs-end]
[inline-code-start]
/** @jsxImportSource npm:hono@4/jsx */
import { Hono } from "npm:hono@4";

const app = new Hono();

app.get("/:slug", (c) => {
  const slug = c.req.param("slug");
  const url = new URL(c.req.path, c.req.url).toString();

  const config = JSON.stringify({
    tenantId: "demo",
    urlId: slug,
    url,
  });

  return c.html(
    <html>
      <body>
        <h1>{slug}</h1>
        <div id="fastcomments-widget"></div>
        <script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
        <script
          dangerouslySetInnerHTML={{
            __html:
              `window.FastCommentsUI(document.getElementById("fastcomments-widget"), ${config});`,
          }}
        />
      </body>
    </html>,
  );
});

export default app.fetch;
[inline-code-end]

## בחר urlId לפני הפריסה

`urlId` מחליט באיזה שרשור נופל תגובה. אם תשאירו אותו ללא הגדרה הוא יכנס לגרסה מנוקה של כתובת ה‑URL של העמוד הנוכחי, שזה בדיוק מה שמשתנה ב‑Val Town: ל‑val יש שם מארח ארוך `*.web.val.run` עד שתת claim תת‑דומיין, לכל סניף יש כתובות משלו, ושינוי שם של עמוד משנה את הנתיב. כל שינוי כזה הופך בשקט לשרשור נפרד וריק, והסימפטום הוא "התגובות שלי נעלמו".

הגדרו אותו למשהו יציב שאתם שולטים בו, כמו ה‑slug של הפוסט או מזהה מסד נתונים, כפי שמופיע למעלה. העבירו גם את `url`, כדי שהודעות האימייל והכלים למודרציה יוכלו לקשר חזרה לעמוד האמיתי.

## שמירת תגובות ללא JavaScript

FastComments מציג שרשור מלא בצד השרת, שבו val יכול להטמיע בתוך בלוק `<noscript>`:

[inline-code-attrs-start title = 'Fallback ללא JavaScript'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

קודדו את הפרמטרים ב‑URL. גרסת ה‑server‑side תומכת בתגובות אנונימיות וכניסות עם חשבון, SSO, ותשובות מקוננות.

---