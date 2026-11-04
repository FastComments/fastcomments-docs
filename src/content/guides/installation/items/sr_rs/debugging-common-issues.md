Ево неких симптома који се често јављају, и уобичајених решења.

### Порука „Ово је демо"

Ово се приказује када сте копирали код виџета са наше почетне странице, који користи наш демо
тенант. Да бисте користили свој тенант, копирајте код виџета са [овде](https://fastcomments.com/auth/my-account/get-acct-code).

### Грешка „FastComments се не може учитати на овом домену"

FastComments мора да зна које домене поседујете како би аутентификовао захтеве везане
за ваш налог. [Погледајте нашу документацију](/guide-multiple-sites.html#add-domains-to-account) да видите како
да решите ову грешку (једноставно додајте тачан поддомен + домен на свој налог).

Имајте на уму да би се ово требало десити само након истека пробног периода. Током пробног периода, сви захтеви
са нових домена биће аутоматски додати на ваш налог.

### Мигрирани коментари се не приказују за прилагођене инсталације

Обично се ово дешава када су увезени коментари везани за `Page ID`, а ви прослеђујете URL
(или ништа, у ком случају се подразумева URL странице).

Ово можете дебаговати тако што ћете [извезти своје коментаре](https://fastcomments.com/auth/my-account/manage-data/export) и погледати колону `URL ID` (тренутно колона `B`).

Уверите се да вредности које видите у колони `URL ID` одговарају вредностима које прослеђујете
конфигурацији виџета као параметар `urlId`.

За даље објашњење, прочитајте нашу документацију [Како су коментари везани за странице и чланке](/guide-customizations-and-configuration.html#url-id).

Ако ништа не помогне, [контактирајте нас](https://fastcomments.com/auth/my-account/help).

### Виџет за коментаре се не приказује

Ако се виџет за коментаре не приказује, проверите Chrome developer конзолу за грешке.

У већини случајева погрешне конфигурације, виџет за коментаре ће барем приказати грешку на страници ако је
у стању да се учита. Ако не видите ништа, то обично указује на грешку у скрипти.

### Жељена конфигурација не ради како се очекује

Пробајте нашу [Chrome екстензију](https://chromewebstore.google.com/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj) да видите која
се конфигурација прослеђује виџету за коментаре. Ако ништа не помогне, направите снимак екрана онога што Chrome екстензија приказује
и [контактирајте нас](https://fastcomments.com/auth/my-account/help).

### Коментари недостају на истом URL-у са различитим hash bang-ом

Подразумевано, FastComments користи URL странице као „канту" (bucket) у којој се чувају коментари. Ако ваши URL-ови садрже `#hashbangs`, а ти `#hashbangs`
не би требало да буду део идентификатора који идентификује нит коментара, можемо једноставно игнорисати вредност hash bang-а, на пример:

[inline-code-attrs-start title = 'Пример игнорисања hash bang-ова'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Имајте на уму да након ове промене треба извршити миграцију за постојеће коментаре. [За то, контактирајте нас.](https://fastcomments.com/auth/my-account/help)

### URL query параметри утичу на виџет

Подразумевано, FastComments користи URL странице као „канту" (bucket) у којој се чувају коментари. Ако ваши URL-ови садрже query параметре
који не би требало да буду део идентификатора који идентификује нит коментара, можемо их једноставно игнорисати, на пример:

[inline-code-attrs-start title = 'Игнорисање query параметара'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Имајте на уму да након ове промене треба извршити миграцију за постојеће коментаре. [За то, контактирајте нас.](https://fastcomments.com/auth/my-account/help)

### Е-поште не стижу

У FastComments улажемо много труда да наша испорука е-поште буде што
поузданија. Међутим, неки добављачи е-поште су познато тешки за поуздану испоруку. Проверите свој spam
фасциклу за поруке од fastcomments.com.

Ако нам [пишете](https://fastcomments.com/auth/my-account/help), обично можемо дати
више увида у разлог зашто не видите е-пошту од нас.