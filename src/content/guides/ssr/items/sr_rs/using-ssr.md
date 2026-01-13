Да бисте користили FastComments SSR, клијент може да преузме HTML са `https://fastcomments.com/ssr/comments` крајње тачке.

Ово се може урадити на више начина.

### У WordPress-у

SSR је омогућен по подразумеваној поставци за кориснике без омогућеног JS-а као резервна опција у WordPress додатку од верзије `3.10.2`.

### На веб-страници

У постојећу апликацију, SSR се може додати помоћу [следећег примера](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr), под претпоставком да је коришћени језик PHP:

[inline-code-attrs-start title = 'PHP пример за SSR'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'PHP пример резервног SSR'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

За пример који користи SSO, [погледајте овде](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr).

### Са претходно рендерованим садржајем

Наш блог се генерише у току build-а, и пружа [добар пример SSR-а са Handlebars](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51).

### Основни параметри

Основни параметри које морате проследити су:
- `tenantId` - Ово вас идентификује као клијента.
- `urlId` - Ово идентификује страницу или чланак за који ће се учитати коментари и дефинише где се чувају.
- `url` - Ово се користи за нотификације и сродне функције да се повежу на нит коментара.

### Прилагођено стилизовање

SSR верзија видгета за коментаре користи исту структуру и рендеринг мотор као и JavaScript верзија.

Стога, сви прилагођени стилови који функционишу за JavaScript видгет за коментаре, функционишу и за SSR.

### Напомене

Са SSR-ом нема JavaScript-а који контролише висину рендерованог контејнера. У прегледачима се може појавити вертикална трака за скроловање за дуге расправе.

Према томе, морате то прилагодити по потреби.