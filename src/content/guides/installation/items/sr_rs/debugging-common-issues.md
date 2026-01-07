Ево неких симптома са којима се често сусрећемо и уобичајених решења.

### Порука "Ово је демо"

Ово се приказује када сте копирали код виџета са наше почетне странице, који користи наш демо
закупац. За коришћење вашег закупца, копирајте код виџета [одавде](https://fastcomments.com/auth/my-account/get-acct-code).

### Грешка "FastComments не може се учитати на овом домену"

FastComments мора знати који домени су у вашем власништву како би аутентификовао захтеве повезане
са вашим налогом. [Погледајте нашу документацију](/guide-multiple-sites.html#add-domains-to-account) да бисте видели како
да решите ову грешку (једноставно додајте тачан поддомен + домен на ваш налог).

Имајте на уму да би се ово требало десити само након истека пробног периода. Током пробног периода, сви захтеви са нових домена
аутоматски ће се додати на ваш налог.

### Мигрирани коментари се не приказују за прилагођене инсталације

Обично се то дешава када су увезени коментари везани за `Page ID`, а ви прослеђујете URL
(или никакву вредност, у ком случају се користи URL странице).

Ово можете дебаговати [извозом ваших коментара](https://fastcomments.com/auth/my-account/manage-data/export) и прегледавањем колоне `URL ID` (тренутно колона `B`).

Осигурајте да су вредности које видите у колони `URL ID` исте вредности које прослеђујете конфигурацији виџета
као параметар `urlId`.

За додатно објашњење, покушајте прочитати нашу [документацију Како су коментари везани за странице и чланке](/guide-customizations-and-configuration.html#url-id).

Ако све друго не успе, [обратите нам се](https://fastcomments.com/auth/my-account/help).

### Виџет за коментаре се не приказује

Ако се виџет за коментаре не приказује, проверите Chrome developer конзолу за грешке.

За већину погрешних конфигурација, виџет за коментаре ће барем приказати грешку на страници ако се
може учитати. Не видети ништа обично указује на скриптну грешку.

### Жељена конфигурација не ради како се очекује

Испробајте наше [Chrome проширење](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) да видите која
конфигурација се прослеђује виџету за коментаре. Ако све пропадне, направите снимак екрана шта chrome проширење каже
и [обратите нам се](https://fastcomments.com/auth/my-account/help).

### Коментари недостају на истом URL-у са различитим hash bang-ом

Према подразумеваним подешавањима, FastComments ће користити URL странице за "bucket" где се коментари чувају. Ако ваши URL-ови укључују `#hashbang`-ове, и ти `#hashbang`-ови
не би требали бити део идентификатора који идентификује нит коментара, можемо једноставно игнорисати hash bang вредност, на пример:

[inline-code-attrs-start title = 'Пример игнорисања hash bang-ова'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
});
</script>
[inline-code-end]

Имајте на уму да ће након ове промене бити потребна миграција за постојеће коментаре. [За то нам се обратите.](https://fastcomments.com/auth/my-account/help)

### URL параметри упита утичу на виџет

Према подразумеваним подешавањима, FastComments ће користити URL странице за "bucket" где се коментари чувају. Ако ваши URL-ови укључују параметре упита
који не би требали бити део идентификатора који идентификује нит коментара, можемо их једноставно игнорисати, на пример:

[inline-code-attrs-start title = 'Игнориши параметре упита'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
});
</script>
[inline-code-end]

Имајте на уму да ће након ове промене бити потребна миграција за постојеће коментаре. [За то нам се обратите.](https://fastcomments.com/auth/my-account/help)

### Не примате е-поште

У FastComments-у улажемо много труда како бисмо осигурали да је испорука наших е-порука што поузданија.
Међутим, неким провајдерима е-поште је нотор тешко поуздано испоручивати. Проверите вашу spam
фасциклу за поруке од fastcomments.com.

Ако нам се [обратите](https://fastcomments.com/auth/my-account/help) обично можемо пружити
више увида зашто можда не видите е-поште од нас.
