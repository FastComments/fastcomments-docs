За да използвате FastComments SSR, клиентът може да извлече HTML от крайна точка `https://fastcomments.com/ssr/comments`.

Това може да се направи по няколко начина.

### С WordPress

SSR е активиран по подразбиране за потребители без включен JS като резервен вариант в плъгина за WordPress от версия `3.10.2`.

### В уеб страница

При вече съществуващо приложение, SSR може да бъде добавен с [следния пример](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr), при условие че използваният език е PHP:

[inline-code-attrs-start title = 'PHP-базиран пример за SSR'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Можем също така да показваме SSR интерфейса само когато потребителят има изключен JS:

[inline-code-attrs-start title = 'PHP-базиран пример за резервен SSR'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

За пример, използващ SSO, [вижте тук](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr).

### С предварително генерирано съдържание

Блогът ни се генерира по време на сборка и предоставя [добър пример за SSR с Handlebars](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51).

### Основните параметри

Основните параметри, които трябва да подадете, са:
- `tenantId` - Това ви идентифицира като клиент.
- `urlId` - Това идентифицира страницата или статията, за която да се заредят коментарите, и определя къде те се запазват.
- `url` - Това се използва за известия и свързани функции за връщане към нишката с коментари.

### Персонализирано оформление

SSR версията на коментарния widget използва същата структура и рендеринг двигател като JavaScript версията.

Следователно, всички персонализирани стилове, които работят за JavaScript widget-а за коментари, работят и за SSR.

### Бележки

При SSR няма JavaScript, който да контролира височината на рендерирания контейнер. В браузърите може да се покаже вертикална лента за превъртане при дълги дискусии.

Следователно трябва да настроите това според желанията си.