Ево неких симптома које често срећемо и уобичајених решења. 

### Порука "Ово је демо"

Ово се приказује када сте копирали код видгета са наше почетне странице, која користи наш demo tenant. Да бисте користили свој tenant, копирајте код видгета са [овде](https://fastcomments.com/auth/my-account/get-acct-code).

### Грешка "FastComments cannot load on this domain"

FastComments мора знати које домене ви поседујете да би аутентификовao захтеве повезане са вашим налогом. [Погледајте нашу документацију](/guide-multiple-sites.html#add-domains-to-account) да бисте видели како да решите ову грешку (једноставно додате тачан субдомен + домен у ваш налог).

Имајте у виду да би ово требало да се појави тек након истека пробног периода. Током пробног периода, сви захтеви са нових домена ће аутоматски бити додати у ваш налог.

### Мигрирани коментари се не приказују за прилагођене инсталације

Обично се ово дешава када су увезени коментари везани за `Page ID`, а ви прослеђујете URL (или нема вредности, у ком случају подразумева се URL странице).

Можете то отстранити тако што ћете [извезти своје коментаре](https://fastcomments.com/auth/my-account/manage-data/export) и погледати колону `URL ID` (тренутно колона `B`).

Уверите се да су вредности које видите у колони `URL ID` исте вредности које прослеђујете конфигурацији видгета као параметар `urlId`.

За додатно објашњење, покушајте да прочитате нашу документацију [How Comments are Tied to Pages and Articles](/guide-customizations-and-configuration.html#url-id).

Ако све друго не успе, [контактирајте нас](https://fastcomments.com/auth/my-account/help).

### Видгет за коментаре се не приказује

Ако се видгет за коментаре не приказује, проверите Chrome конзолу за програмере за грешке.

За већину конфигурационих грешака, видгет за коментаре ће барем приказати поруку о грешци на страници ако успе да се учита. Ако ништа не видите, обично је то показатељ грешке у скрипти.

### Желјена конфигурација не ради како се очекује

Пробајте нашу Chrome екстензију да видите коју конфигурацију видгет коментара добија. Ако ништа не помаже, направите снимак екрана онога што Chrome екстензија показује и [контактирајте нас](https://fastcomments.com/auth/my-account/help).

### Коментари недостају на истом URL-у са различитим hash bang-овима

По подразумеваној поставци, FastComments ће користити URL странице за "канту" у којој се чувају коментари. Ако ваши URL-ови садрже `#hashbangs`, а ови `#hashbangs` не би требало да буду део идентификатора који одређује нит коментара, можемо једноставно занемарити вредност hash banga, на пример:

[inline-code-attrs-start title = 'Пример игнорисања hash-bang вредности'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Имајте у виду да ће, након ове промене, морати да се изврши миграција постојећих коментара. [За то, контактирајте нас.](https://fastcomments.com/auth/my-account/help)

### URL параметри упита који утичу на видгет

По подразумеваној поставци, FastComments ће користити URL странице за "канту" у којој се чувају коментари. Ако ваши URL-ови садрже параметре упита који не би требало да буду део идентификатора који одређује нит коментара, можемо их једноставно занемарити, на пример:

[inline-code-attrs-start title = 'Пример игнорисања параметара упита'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Имајте у виду да ће, након ове промене, морати да се изврши миграција постојећих коментара. [За то, контактирајте нас.](https://fastcomments.com/auth/my-account/help)

### Не примате е-поруке

У FastComments-у улажемо много труда да наша испорука е-порука буде што поузданија. Међутим, неки провајдери е-поште су познато тешки за поуздану доставу. Проверите фолдер за нежељену пошту за поруке од fastcomments.com.

Ако [нас контактирате](https://fastcomments.com/auth/my-account/help), обично можемо пружити више информација зашто можда не примате е-поруке од нас.