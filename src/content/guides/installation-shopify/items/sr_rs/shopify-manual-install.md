Ако не можете да инсталирате апликацију из [Shopify App Store](https://apps.shopify.com/fastcomments), и даље можете да додате FastComments уређивањем своје теме. Овај пут је користан када желите да повежете FastComments tenant који већ поседујете, или када уграђујете на Shopify продајни простор где апликација није опција.

Апликацијска инсталација је подржан пут за већину продавница. Користите овај начин само ако апликација није погодна.

### Корак 1: Онемогућите уграђене коментаре Shopify-а

У свом Shopify администраторском панелу идите на **Blog posts > Manage blogs**, отворите сваки блог и у десном панелу подесите **Comments are disabled**. Сачувајте.

Ово спречава да Shopify-ев уграђени систем коментара буде видљив заједно са FastComments.

### Корак 2: Отворите шаблон теме блога

У свом Shopify администраторском панелу:

1. Идите на **Онлајн продавница > Теме**.
2. Под вашом текућом темом кликните **Акције > Уреди код**.
3. У прегледачу фајлова са леве стране отворите **Sections** и кликните `main-article.liquid`.

Ово је шаблон који Shopify приказује за појединачну објаву на блогу.

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

Замените `"demo"` својим Tenant ID-јем са [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Кликните **Сачувај**.

### Корак 4: Овластите домен ваше продавнице

Отворите објаву на блогу на вашој уживо продавници. Ако уместо видгета за коментаре видите грешку овлашћења, FastComments мора да зна да је вашој продавници дозвољено да користи овај tenant. Погледајте [Грешке домена](/guide-installation-shopify.html#shopify-domain-errors).

### Додавање FastComments на друге странице

Исти исечак ради у сваком Liquid шаблону, укључујући странице производа, прилагођене странице и почетну страницу. Налепите га тамо где желите да се коментари појаве и прилагодите `urlId` ако желите стабилни идентификатор по страници (на пример, `urlId: "{{ product.id }}"` у шаблону производа).