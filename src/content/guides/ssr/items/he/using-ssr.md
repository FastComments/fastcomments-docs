כדי להשתמש ב‑FastComments SSR, הלקוח יכול לשלוף HTML מהנקודת הקצה `https://fastcomments.com/ssr/comments`.

ניתן לעשות זאת בכמה דרכים.

### עם WordPress

SSR מופעל כברירת מחדל עבור משתמשים שאין להם JS מופעל כפתרון גיבוי בפלאגין של WordPress מאז גרסה `3.10.2`.

### בדף אינטרנט

עם יישום קיים כבר, ניתן להוסיף SSR באמצעות [הדוגמה הבאה](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr), בהנחה שהשפה שבה משתמשים היא PHP:

[inline-code-attrs-start title = 'דוגמת SSR מבוססת PHP'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
$tenantId = rawurlencode("my-tenant-id");
$urlId = rawurlencode("my-page-url-article-id");
$url = rawurlencode("my-page-url");

$fastcomments_url = "https://fastcomments.com/ssr/comments?tenantId=$tenantId&urlId=$urlId&url=$url";
?>
<iframe
    src="<?php echo $fastcomments_url; ?>"
    horizontalscrolling="no"
    allowtransparency="true"
    frameborder="0"
    title="FastComments"
    width="100%"
    height="1500px"
    style= "width: 1px !important; min-width: 100% !important; border: none !important; overflow: hidden !important;"
></iframe>
[inline-code-end]

ניתן גם להציג את ממשק ה‑SSR רק כאשר למשתמש ה‑JS מושבת:

[inline-code-attrs-start title = 'דוגמת גיבוי SSR מבוססת PHP'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
$tenantId = rawurlencode("my-tenant-id");
$urlId = rawurlencode("my-page-url-article-id");
$url = rawurlencode("my-page-url");

$fastcomments_url = "https://fastcomments.com/ssr/comments?tenantId=$tenantId&urlId=$urlId&url=$url";
?>
<noscript>
    <iframe
        src="<?php echo $fastcomments_url; ?>"
        horizontalscrolling="no"
        allowtransparency="true"
        frameborder="0"
        title="FastComments"
        width="100%"
        height="1500px"
        style= "width: 1px !important; min-width: 100% !important; border: none !important; overflow: hidden !important;"
    ></iframe>
</noscript>
[inline-code-end]

לדוגמה שמשתמשת ב‑SSO, [ראו כאן](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr).

### עם תוכן שנוצר מראש

הבלוג שלנו נוצר בזמן הבנייה, והוא מספק [דוגמה טובה ל‑SSR עם Handlebars](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51).

### הפרמטרים הבסיסיים

הפרמטרים הבסיסיים שעליכם להעביר הם:
- `tenantId` - מזהה אתכם כלקוח.
- `urlId` - מזהה את הדף או המאמר לטעינת התגובות עבורו, וקובע היכן נשמרות התגובות.
- `url` - משמש להודעות ולתכונות קשורות כדי לקשר חזרה לשרשור התגובות.

### עיצוב מותאם אישית

גרסת ה‑SSR של ווידג'ט התגובות משתמשת באותו מבנה ובאותו מנוע רינדור כמו הווידג'ט של JavaScript.

לכן, כל עיצוב מותאם אישית שעובד עבור ווידג'ט התגובות ב‑JavaScript יעבוד גם עבור SSR. 

### הערות

ב‑SSR אין JavaScript שולט בגובה המיכל המוצג. בדפדפנים, ייתכן שיופיע פס גלילה אנכית עבור דיונים ארוכים.

לכן, עליכם לכוונן זאת לפי הצורך.