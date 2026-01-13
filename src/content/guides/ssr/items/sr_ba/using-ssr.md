---
Да бисте користили FastComments SSR, клијент може преузети HTML са `https://fastcomments.com/ssr/comments` крајње тачке.

Ово се може урадити на неколико начина.

### Са WordPress-ом

SSR је омогућен по дифолту за кориснике који немају омогућен JS као резервна опција у WordPress додатку од верзије `3.10.2`.

### У веб-страници

У већ постојећој апликацији, SSR се може додати помоћу [слиједећег примјера](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr), под условом да је кориштени језик PHP:

[inline-code-attrs-start title = 'PHP-примјер за SSR'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Такође можемо приказивати SSR кориснички интерфејс само када корисник има онемогућен JS:

[inline-code-attrs-start title = 'PHP-примјер за резервни режим SSR'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

За примјер који користи SSO, погледајте [овдје](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr).

### Са унапред рендерованим садржајем

Наш блог се генерише током процеса изградње и пружа [добар примјер SSR-а са Handlebars](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51).

### Основни параметри

Основни параметри које треба прослиједити су:
- `tenantId` - Ово вас идентификује као клијента.
- `urlId` - Ово идентификује страницу или чланак за које се учитавају коментари, и дефинише гдје се чувају.
- `url` - Ово се користи за обавијештења и сродне функције како би се везало назад на нит коментара.

### Прилагођени стилови

SSR верзија коментарског видџета користи исту структуру и рендеринг мотор као и JavaScript верзија.

Дакле, сав прилагођени стил који ради за JavaScript видџет за коментаре ради и за SSR. 

### Напомене

Код SSR-а нема JavaScript-а који контролише висину исцртаног контејнера. У прегледачима се може појавити вертикална трака за скроловање за дуге дискусије.

Стога, морате то прилагодити по потреби.

---