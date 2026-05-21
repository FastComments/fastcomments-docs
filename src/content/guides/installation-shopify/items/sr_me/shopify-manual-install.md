Ако не можете да инсталирате апликацију из [Shopify App Store app](https://apps.shopify.com/fastcomments), и даље можете додати FastComments уређивањем ваше теме. Овај пут је користан када желите да повежете FastComments tenant који већ поседујете, или када уграђујете на Shopify продавницу где апликација није опција.

Инсталација преко апликације је подржан пут за већину продавница. Користите овај начин само ако апликација не одговара.

### Корак 1: Онемогућите уграђене коментаре Shopify-а

У вашем Shopify администраторском панелу идите на **Блог постови > Управљање блоговима**, отворите сваки блог и подесите **Comments are disabled** у десном панелу. Сачувајте.

Ово спречава да Shopify-јев уграђени систем коментара буде приказан заједно са FastComments.

### Корак 2: Отворите шаблон теме за блог

У вашем Shopify администраторском панелу:

1. Идите на **Онлајн продавница > Теме**.
2. Под вашом тренутном темом кликните **Акције > Уреди код**.
3. У претраживачу фајлова са леве стране отворите **Sections** и кликните `main-article.liquid`.

Ово је шаблон који Shopify приказује за један блог чланак.

### Корак 3: Залепите FastComments исјечак

Скролујте до отприлике линије 100 фајла `main-article.liquid`, одмах након затварајућег `</div>` тела чланка. Залепите следећи исјечак:

[inline-code-attrs-start title = 'Shopify FastComments исјечак'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Замените `"demo"` својим Tenant ID-јем са [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Кликните **Save**.

### Корак 4: Овластите домен ваше продавнице

Отворите пост на блогу на вашој живој продавници. Ако уместо виджета за коментаре видите грешку за ауторизацију, FastComments треба да зна да је вашa продавница овлашћена да користи овај tenant. Види [Domain Errors](/guide-installation-shopify.html#shopify-domain-errors).

### Додавање FastComments на друге странице

Исти исјечак ради на било ком Liquid шаблону, укључујући странице производа, прилагођене странице и почетну страницу. Залепите га тамо где желите да се коментари појаве и прилагодите `urlId` ако желите стабилан идентификатор по страници (на пример, `urlId: "{{ product.id }}"` на шаблону производа).