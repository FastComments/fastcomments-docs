Если вы не можете установить [приложение Shopify App Store](https://apps.shopify.com/fastcomments), вы всё ещё можете добавить FastComments, отредактировав вашу тему. Этот путь полезен, когда вы хотите подключить уже существующий тенант FastComments или когда вы встраиваете на витрину Shopify, где приложение недоступно.

Установка через приложение — поддерживаемый путь для большинства магазинов. Используйте этот метод только если приложение вам не подходит.

### Шаг 1: Отключите встроенные комментарии Shopify

В админке Shopify перейдите в **Blog posts > Manage blogs**, откройте каждый блог и в правой панели установите **Comments are disabled**. Сохраните.

Это остановит показ встроенных комментариев Shopify рядом с FastComments.

### Шаг 2: Откройте шаблон темы блога

В админке Shopify:

1. Перейдите в **Online Store > Themes**.
2. Под вашей текущей темой нажмите **Actions > Edit code**.
3. В обозревателе файлов слева откройте **Sections** и нажмите `main-article.liquid`.

Это шаблон, который Shopify рендерит для одной статьи блога.

### Шаг 3: Вставьте сниппет FastComments

Прокрутите примерно до строки 100 в `main-article.liquid`, сразу после закрывающего тега `</div>` тела статьи. Вставьте следующий сниппет:

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

Замените `"demo"` на ваш собственный Tenant ID с [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Нажмите **Сохранить**.

### Шаг 4: Авторизуйте домен вашего магазина

Откройте запись блога на вашем живом магазине. Если вместо виджета комментариев вы видите ошибку авторизации, FastComments нужно знать, что вашему магазину разрешено использовать этот тенант. См. [Ошибки домена](/guide-installation-shopify.html#shopify-domain-errors).

### Добавление FastComments на другие страницы

Тот же сниппет работает в любом шаблоне Liquid, включая страницы товаров, кастомные страницы и главную страницу. Вставьте его туда, где вы хотите, чтобы появлялись комментарии, и при необходимости отрегулируйте `urlId`, если вы хотите стабильный идентификатор для каждой страницы (например, `urlId: "{{ product.id }}"` в шаблоне товара).