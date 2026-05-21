Ако не можете инсталирати [Shopify App Store апликацију](https://apps.shopify.com/fastcomments), и даље можете додати FastComments уређивањем ваше теме. Овај пут је користан када желите повезати FastComments tenant који већ поседујете, или када уграђујете на Shopify продавницу где апликација није опција.

Инсталација преко апликације је подржани начин за већину продавница. Користите овај метод само ако апликација није погодна.

### Корак 1: Онемогућите уграђене коментаре Shopify-а

У вашем Shopify администраторском панелу идите на **Blog posts > Manage blogs**, отворите сваки блог и у десном панелу подесите **Comments are disabled**. Сачувајте.

Ово спречава да Shopify-ев уграђени систем коментирања буде видљив заједно са FastComments.

### Корак 2: Отворите шаблон теме блога

У вашем Shopify администраторском панелу:

1. Идите на **Онлајн продавница > Теме**.
2. Под вашом тренутном темом кликните **Акције > Уреди код**.
3. У прегледачу фајлова са леве стране отворите **Секције** и кликните `main-article.liquid`.

Ово је шаблон који Shopify приказује за појединачни чланак на блогу.

### Корак 3: Налепите FastComments исечак

Скролујте до отприлике линије 100 у `main-article.liquid`, одмах након затварајућег `</div>` тела чланка. Налепите следећи исечак:

[inline-code-attrs-start title = 'Shopify FastComments исечак'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Замените `"demo"` вашим Tenant ID-јем са [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Кликните **Сачувај**.

### Корак 4: Овластите домен ваше продавнице

Отворите чланак на блогу на вашој живој продавници. Ако уместо виджета за коментаре видите грешку овлашћења, потребно је да FastComments зна да је ваша продавница овлашћена да користи овај tenant. Погледајте [Грешке домена](/guide-installation-shopify.html#shopify-domain-errors).

### Додавање FastComments на друге странице

Исти исечак ради у било ком Liquid шаблону, укључујући странице производа, прилагођене странице и почетну страницу. Налепите га тамо где желите да се коментари појаве и прилагодите `urlId` ако желите стабилан идентификатор по страници (на пример, `urlId: "{{ product.id }}"` у шаблону производа).