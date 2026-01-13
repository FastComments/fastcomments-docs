Да бисте користили FastComments SSR, клијент може да преузме HTML са `https://fastcomments.com/ssr/comments` крајње тачке.

Ово се може урадити на више начина.

### Са WordPress-ом

SSR је омогућен по дефаулту за кориснике без укљученог JS-а као резервна опција у WordPress додатку од верзије `3.10.2`.

### На веб страници

Код већ постојеће апликације, SSR се може додати помоћу [следећег примера](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr), под претпоставком да је коришћени језик PHP:

[inline-code-attrs-start title = 'PHP примјер за SSR'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Такође можемо приказати SSR кориснички интерфејс само када корисник има онемогућен JS:

[inline-code-attrs-start title = 'PHP примјер резервне опције за SSR'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

### Са унапред генерисаним садржајем

Наш блог се генерише током процеса изградње и пружа [добар пример SSR-а са Handlebars](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51).

### Основни параметри

Основни параметри које треба проследити су:
- `tenantId` - Ово вас идентификује као купца.
- `urlId` - Ово идентификује страницу или чланак за који се учитавају коментари и одређује где се они чувају.
- `url` - Ово се користи за обавештења и сродне функције да повежу назад на нит коментара.

### Прилагођени стилови

SSR верзија видџета за коментаре користи исту структуру и механизам рендеровања као и JavaScript верзија.

Према томе, сва прилагођена стилизација која ради за JavaScript видџет за коментаре ради и за SSR. 

### Напомене

Код SSR-а нема JavaScript-а који контролише висину рендерованог контејнера. У прегледачима се за дуге дискусије може појавити вертикална трака за скроловање.

Стога морате прилагодити ово по жељи.