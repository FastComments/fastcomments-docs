בעמוד אינדקס, אל תציג וידג'ט ספירת תגובות אחד לכל שורה. זהו בקשה אחת לכל פוסט. השתמש בספירה המרובה, אשר לוקחת בקשה אחת לכל העמוד.

סמן כל שורה עם ה-`urlId` שהשרשור שלה משתמש, ואז טען את הווידג'ט המרובה פעם אחת:

[inline-code-attrs-start title = 'ספירות תגובות מרובות בעמוד אינדקס'; type='javascript' inline-code-attrs-end]
[inline-code-start]
<ul>
  {posts.map((post) => (
    <li>
      <a href={post.slug}>{post.title}</a>{" "}
      <span class="fast-comments-count" data-fast-comments-url-id={post.slug}></span>
    </li>
  ))}
</ul>

<script
  dangerouslySetInnerHTML={{
    __html: `window.FastCommentsBulkCountConfig = ${
      JSON.stringify({ tenantId: TENANT_ID })
    };`,
  }}
/>
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>
[inline-code-end]

הסקריפט מוצא כל אלמנט `.fast-comments-count` בעמוד וממלא את ספירתו.

`data-fast-comments-url-id` חייב להתאים ל-`urlId` שהווידג'ט של הפוסט משתמש בו. אם הווידג'ט משתמש ב‑slug, הסמן משתמש ב‑slug. חוסר התאמה מציג אפס על שרשור שיש לו תגובות.

הסקריפט בודק באופן מחזורי את `window.FastCommentsBulkCountConfig`, ולכן לא משנה אם אתה מגדיר את ההגדרה לפני או אחרי תג הסקריפט.