Овдје су неки симптоми са којима често наилазимо и уобичајена рјешења. 

### Порука "Ово је демо"

Ово се приказује када сте копирали код видгета са наше почетне странице, која користи наш демо tenant. Да бисте користили свој tenant, копирајте код видгета из [овде](https://fastcomments.com/auth/my-account/get-acct-code).

### Грешка "FastComments не може да се учита на овом домену"

FastComments мора знати које домене посједујете како би аутентификовао захтјеве повезане са вашим налогом. [Погледајте нашу документацију](/guide-multiple-sites.html#add-domains-to-account) да бисте видјели како ријешити ову грешку (једноставно додајте тачан субдомен + домен у ваш налог).

Имајте на уму да би ово требало да се деси само након што пробни период истекне. Током пробног периода, сви захтјеви са нових домена ће бити аутоматски додати у ваш рачун.

### Мигрирани коментари се не приказују за прилагођене инсталације

Обично се ово дешава када увезени коментари имају везу са `Page ID`, а ви просљеђујете URL (или нема вриједности, у ком случају подразумевано је URL странице).

Можете ово дебуговати тако што ћете [извезти своје коментаре](https://fastcomments.com/auth/my-account/manage-data/export) и погледати колону `URL ID` (тренутно колона `B`).

Увјерите се да су вриједности које видите у колони `URL ID` исте вриједности које просљеђујете конфигурацији видгета као параметар `urlId`.

За додатно објашњење, покушајте прочитати нашу документацију [Како су коментари везани за странице и чланке](/guide-customizations-and-configuration.html#url-id).

Ако ни то не помогне, [обратите нам се](https://fastcomments.com/auth/my-account/help).

### Видгет коментара се не приказује

Ако се видгет коментара не приказује, провјерите Chrome конзолу за програмере за грешке.

За већину погрешних конфигурација, видгет коментара ће барем показати грешку на страници ако је у стању да се учита. Ничега видјети обично указује на скриптну грешку.

### Жељена конфигурација не ради како се очекује

Покушајте нашу [Chrome екстензију](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) да видите коју конфигурацију видгет коментара добија. Ако ништа не успије, направите снимак екрана онога што екстензија за Chrome приказује и [обратите нам се](https://fastcomments.com/auth/my-account/help).

### Коментари недостају на истом URL-у са различитим хеш-банговима

По подразумјевaњу, FastComments ће користити URL странице као "канту" гдје се коментари чувају. Ако ваши URL-ови садрже `#hashbangs`, и ти `#hashbangs` не би требали бити дио идентификатора који идентификује нит коментара, можемо једноставно занемарити вриједност хеш-банга, на примјер:

[inline-code-attrs-start title = 'Примјер занемаривања хеш-бангова'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
}];
</script>
[inline-code-end]

Имајте на уму да ће након ове промјене морати бити извршена миграција постојећих коментара. [За то нам се обратите.](https://fastcomments.com/auth/my-account/help)

### Параметри упита у URL-у који утичу на видгет

По подразумјевaњу, FastComments ће користити URL странице као "канту" гдје се коментари чувају. Ако ваши URL-ови садрже параметре упита који не би требали бити дио идентификатора који идентификује нит коментара, можемо их једноставно занемарити, на примјер:

[inline-code-attrs-start title = 'Занемаривање параметара упита'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
}];
</script>
[inline-code-end]

Имајте на уму да ће након ове промјене морати бити извршена миграција постојећих коментара. [За то нам се обратите.](https://fastcomments.com/auth/my-account/help)

### Не примате е-пошту

У FastComments-у улажемо пуно рада како бисмо осигурали да је испорука наших мејлова што поузданија. Међутим, неки провајдери е-поште су познато тешки за поуздану доставу. Провјерите фолдер за нежељену пошту за поруке од fastcomments.com.

Ако нам се [обратите](https://fastcomments.com/auth/my-account/help), обично можемо пружити више увида у разлоге због којих можда не примате мејлове од нас.