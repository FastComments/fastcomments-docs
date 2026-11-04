Если вы не можете установить приложение из [Shopify App Store](https://apps.shopify.com/fastcomments), вы всё равно можете добавить FastComments, отредактировав свою тему. Этот способ полезен, когда вы хотите подключить уже принадлежащий вам тенант FastComments или когда вы встраиваете комментарии на витрину Shopify, где использование приложения невозможно.

Установка через приложение — это поддерживаемый способ для большинства магазинов. Используйте этот способ только в том случае, если приложение вам не подходит.

### Шаг 1: Отключите встроенные комментарии Shopify

В админ-панели Shopify перейдите в **Blog posts > Manage blogs**, откройте каждый блог и в правой панели выберите **Comments are disabled**. Сохраните.

Это предотвратит отображение встроенных комментариев Shopify вместе с FastComments.

### Шаг 2: Откройте шаблон темы блога

В админ-панели Shopify:

1. Перейдите в **Online Store > Themes**.
2. Под текущей темой нажмите **Actions > Edit code**.
3. В файловом браузере слева откройте **Sections** и нажмите `main-article.liquid`.

Это шаблон, который Shopify использует для отображения отдельной статьи блога.

### Шаг 3: Вставьте сниппет FastComments

Прокрутите примерно до 100-й строки файла `main-article.liquid`, сразу после закрывающего `</div>` тела статьи. Вставьте следующий сниппет:

[inline-code-attrs-start title = 'Сниппет FastComments для Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        urlId: window.location.pathname
    }];
</script>
[inline-code-end]

Замените `"demo"` на ваш собственный Tenant ID со страницы [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Нажмите **Save**.

### Шаг 4: Авторизуйте домен вашего магазина

Откройте запись в блоге на вашем рабочем магазине. Если вместо виджета комментариев вы видите ошибку авторизации, FastComments должен знать, что вашему магазину разрешено использовать этот тенант. См. [Ошибки домена](/guide-installation-shopify.html#shopify-domain-errors).

### Добавление FastComments на другие страницы

Тот же сниппет работает на любом шаблоне Liquid, включая страницы товаров, пользовательские страницы и главную страницу. Вставьте его там, где вы хотите видеть комментарии, и при необходимости измените `urlId`, чтобы получить стабильный идентификатор для каждой страницы (например, `urlId: "{{ product.id }}"` в шаблоне товара).