Чтобы использовать FastComments SSR, клиент может получить HTML с конечной точки `https://fastcomments.com/ssr/comments`.

Это можно сделать несколькими способами.

### С WordPress

SSR включён по умолчанию как запасной вариант в плагине WordPress для пользователей с отключённым JS, начиная с версии `3.10.2`.

### На веб-странице

В уже существующее приложение можно добавить SSR с помощью [следующего примера](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr), при условии, что язык используется PHP:

[inline-code-attrs-start title = 'Пример SSR на PHP'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Мы также можем показывать интерфейс SSR только в случае, если у пользователя отключён JS:

[inline-code-attrs-start title = 'Пример резервного SSR на PHP'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Для примера с использованием SSO, [см. здесь](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr).

### С предварительно отрендеренным контентом

Наш блог генерируется во время сборки и содержит [хороший пример SSR с Handlebars](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51).

### Базовые параметры

Базовые параметры, которые нужно передать:
- `tenantId` - Это идентифицирует вас как клиента.
- `urlId` - Определяет страницу или статью, для которой загружаются комментарии, и место их сохранения.
- `url` - Используется для уведомлений и связанных функций, чтобы ссылаться обратно на поток комментариев.

### Пользовательские стили

SSR-версия виджета комментариев использует ту же структуру и движок рендеринга, что и JavaScript-версия.

Следовательно, все пользовательские стили, которые работают для JavaScript-виджета комментариев, работают и для SSR. 

### Примечания

При SSR отсутствует JavaScript, управляющий высотой отрендеренного контейнера. В браузерах для длинных обсуждений может появляться вертикальная полоса прокрутки.

Соответственно, вам нужно настроить это по своему усмотрению.

---