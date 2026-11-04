Якщо ви не можете встановити застосунок із [Shopify App Store](https://apps.shopify.com/fastcomments), ви все одно можете додати FastComments, відредагувавши свою тему. Цей шлях корисний, коли ви хочете підключити вже наявний у вас тенант FastComments, або коли ви вбудовуєте віджети на вітрину Shopify, де застосунок недоступний.

Встановлення через застосунок є підтримуваним шляхом для більшості магазинів. Використовуйте цей спосіб, лише якщо застосунок вам не підходить.

### Крок 1: Вимкніть нативні коментарі Shopify

В адмінпанелі Shopify перейдіть до **Blog posts > Manage blogs**, відкрийте кожен блог і встановіть **Comments are disabled** на панелі праворуч. Збережіть.

Це запобігає показу вбудованих коментарів Shopify разом із FastComments.

### Крок 2: Відкрийте шаблон теми блогу

В адмінпанелі Shopify:

1. Перейдіть до **Online Store > Themes**.
2. Під вашою поточною темою натисніть **Actions > Edit code**.
3. У файловому браузері ліворуч відкрийте **Sections** і натисніть `main-article.liquid`.

Це шаблон, який Shopify відображає для окремої статті блогу.

### Крок 3: Вставте фрагмент FastComments

Прокрутіть приблизно до 100-го рядка `main-article.liquid`, одразу після закриваючого `</div>` тіла статті. Вставте наступний фрагмент:

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

Замініть `"demo"` на власний Tenant ID зі сторінки [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Натисніть **Save**.

### Крок 4: Авторизуйте домен вашого магазину

Відкрийте запис у блозі на вашому працюючому магазині. Якщо замість віджета коментарів ви бачите помилку авторизації, FastComments потрібно знати, що вашому магазину дозволено використовувати цей тенант. Дивіться [Помилки домену](/guide-installation-shopify.html#shopify-domain-errors).

### Додавання FastComments на інші сторінки

Цей самий фрагмент працює на будь-якому шаблоні Liquid, включно зі сторінками товарів, користувацькими сторінками та головною сторінкою. Вставте його там, де ви хочете бачити коментарі, і за потреби змініть `urlId`, щоб отримати стабільний ідентифікатор для кожної сторінки (наприклад, `urlId: "{{ product.id }}"` у шаблоні товару).