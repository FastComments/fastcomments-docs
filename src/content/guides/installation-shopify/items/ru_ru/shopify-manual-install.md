---
Если вы не можете установить [приложение Shopify App Store](https://apps.shopify.com/fastcomments), вы всё равно можете добавить FastComments, отредактировав тему. Этот путь полезен, когда вы хотите подключить уже существующий тенант FastComments или встроить его на витрину Shopify, где установка приложения недоступна.

The app-based install is the supported path for most stores. Reach for this only if the app doesn't fit.

### Step 1: Disable Shopify's native comments

В админке Shopify перейдите в **Записи блога > Управление блогами**, откройте каждый блог и в правой панели установите **Комментарии отключены**. Сохраните.

Это предотвращает отображение встроенных комментариев Shopify вместе с FastComments.

### Step 2: Open the blog theme template

В админке Shopify:

1. Перейдите в **Интернет-магазин > Темы**.
2. Под вашей текущей темой нажмите **Действия > Редактировать код**.
3. В проводнике файлов слева откройте **Sections** и кликните `main-article.liquid`.

Это шаблон, который Shopify использует для отображения отдельной записи в блоге.

### Step 3: Paste the FastComments snippet

Прокрутите примерно до строки 100 в `main-article.liquid`, сразу после закрывающего тега `</div>` тела статьи. Вставьте следующий фрагмент:

[inline-code-attrs-start title = 'Фрагмент FastComments для Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Replace "demo" with your own Tenant ID from [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Нажмите **Сохранить**.

### Step 4: Authorize your shop domain

Откройте запись блога в публичном магазине. Если вместо виджета комментариев вы видите ошибку авторизации, FastComments должен знать, что вашему магазину разрешено использовать этот тенант. См. [Ошибки домена](/guide-installation-shopify.html#shopify-domain-errors).

### Adding FastComments to other pages

The same snippet works on any Liquid template, including product pages, custom pages, and the home page. Paste it where you want comments to appear and adjust `urlId` if you want a stable identifier per page (for example, `urlId: "{{ product.id }}"` on a product template).

---