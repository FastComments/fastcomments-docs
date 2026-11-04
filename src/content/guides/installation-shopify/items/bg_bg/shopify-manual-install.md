Ако не можете да инсталирате [приложението от Shopify App Store](https://apps.shopify.com/fastcomments), все пак можете да добавите FastComments, като редактирате вашата тема. Този път е полезен, когато искате да свържете FastComments наемател (tenant), който вече притежавате, или когато вграждате в Shopify витрина, където приложението не е опция.

Инсталацията чрез приложението е поддържаният път за повечето магазини. Използвайте този само ако приложението не е подходящо.

### Стъпка 1: Деактивирайте вградените коментари на Shopify

В Shopify админ панела отидете на **Blog posts > Manage blogs**, отворете всеки блог и задайте **Comments are disabled** в десния панел. Запазете.

Това спира вграденото коментиране на Shopify да се показва заедно с FastComments.

### Стъпка 2: Отворете шаблона на блога в темата

В Shopify админ панела:

1. Отидете на **Online Store > Themes**.
2. Под текущата си тема щракнете **Actions > Edit code**.
3. В файловия браузър отляво отворете **Sections** и щракнете `main-article.liquid`.

Това е шаблонът, който Shopify изобразява за отделна статия в блога.

### Стъпка 3: Поставете FastComments снипета

Превъртете до около ред 100 на `main-article.liquid`, точно след затварящия `</div>` на тялото на статията. Поставете следния снипет:

[inline-code-attrs-start title = 'Shopify FastComments снипет'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Заменете `"demo"` със собствения си Tenant ID от [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Щракнете **Save**.

### Стъпка 4: Авторизирайте домейна на магазина си

Отворете публикация в блога на живия си магазин. Ако вместо уиджета за коментари видите грешка при авторизация, FastComments трябва да знае, че вашият магазин има право да използва този наемател. Вижте [Грешки с домейни](/guide-installation-shopify.html#shopify-domain-errors).

### Добавяне на FastComments към други страници

Същият снипет работи на всеки Liquid шаблон, включително страници с продукти, персонализирани страници и началната страница. Поставете го там, където искате да се показват коментарите, и коригирайте `urlId`, ако искате стабилен идентификатор за всяка страница (например `urlId: "{{ product.id }}"` в шаблон за продукт).