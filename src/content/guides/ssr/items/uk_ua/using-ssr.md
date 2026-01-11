Щоб використовувати FastComments SSR, клієнт може отримати HTML із кінцевої точки `https://fastcomments.com/ssr/comments`.

Це можна зробити кількома способами.

### У WordPress

SSR увімкнено за замовчуванням у плагіні WordPress як запасний варіант для користувачів без підтримки JS, починаючи з версії `3.10.2`.

### На веб-сторінці

У вже існуючому додатку SSR можна додати за допомогою [наступного прикладу](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr), якщо використовується мова PHP:

[inline-code-attrs-start title = 'Приклад SSR на PHP'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Можна також показувати інтерфейс SSR лише коли у користувача вимкнено JS:

[inline-code-attrs-start title = 'Приклад запасного варіанту SSR на PHP'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Для прикладу з використанням SSO дивіться [тут](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr).

### З попередньо згенерованим контентом

Наш блог генерується під час збірки і містить [хороший приклад SSR з Handlebars](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51).

### Основні параметри

Основні параметри, які потрібно передати:
- `tenantId` - Це ідентифікує вас як клієнта.
- `urlId` - Ідентифікує сторінку або статтю, для якої потрібно завантажити коментарі, і визначає, де вони зберігаються.
- `url` - Використовується для сповіщень та пов'язаних функцій, щоб створити посилання назад на потік коментарів.

### Користувацьке стилювання

SSR-версія віджета коментарів використовує ту ж структуру та механізм рендерингу, що й JavaScript-версія.

Отже, усі користувацькі стилі, які працюють для JavaScript-віджета коментарів, працюватимуть і для SSR. 

### Примітки

У SSR немає JavaScript, що контролює висоту відображуваного контейнера. У браузерах може з'явитися вертикальна смужка прокрутки для довгих обговорень.

Тому потрібно налаштувати це за потреби.

---