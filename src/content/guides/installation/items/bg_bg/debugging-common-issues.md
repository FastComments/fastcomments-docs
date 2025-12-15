Ето някои симптоми, които често срещаме, и типични решения.

### Съобщение "This is a demo"

Това се показва, когато сте копирали кода на уиджета от нашата начална страница, която използва нашия демо тенант. За да използвате вашия тенант, копирайте кода на уиджета [оттук](https://fastcomments.com/auth/my-account/get-acct-code).

### Грешка "FastComments cannot load on this domain"

FastComments трябва да знае кои домейни са ваши, за да удостовери заявки, свързани с вашия акаунт. [Вижте нашата документация](/guide-multiple-sites.html#add-domains-to-account) за да разберете как да разрешите тази грешка (просто добавете точния поддомейн + домейн към вашия акаунт).

Имайте предвид, че това трябва да се случи само след изтичане на пробния период. По време на пробния период всички заявки от нови домейни автоматично ще бъдат добавени към вашия акаунт.

### Мигрираните коментари не се показват за персонализирани инсталации

Обикновено това се случва, когато импортираните коментари са свързани с `Page ID`, а вие подавате URL (или никаква стойност, в който случай по подразбиране се използва URL на страницата).

Можете да отстраните това, [експортирайки вашите коментари](https://fastcomments.com/auth/my-account/manage-data/export) и преглеждайки колоната `URL ID` (в момента Колона `B`).

Уверете се, че стойностите, които виждате в колоната `URL ID`, са същите стойности, които подавате към конфигурацията на уиджета като параметър `urlId`.

За допълнително обяснение, опитайте да прочетете нашата [документация за това как коментарите са свързани със страници и статии](/guide-customizations-and-configuration.html#url-id).

Ако нищо друго не помогне, [свържете се с нас](https://fastcomments.com/auth/my-account/help).

### Уиджетът за коментари не се показва

Ако уиджетът за коментари не се показва, проверете конзолата за разработчици на Chrome за грешки.

При повечето грешни конфигурации уиджетът за коментари поне ще покаже грешка на страницата, ако може да се зареди. Да не виждате нищо обикновено е знак за грешка в скрипта.

### Желаната конфигурация не работи както се очаква

Опитайте нашето [Chrome разширение](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US), за да видите каква конфигурация се подава на уиджета за коментари. Ако нищо не помогне, направете екранна снимка на това, което показва Chrome разширението и [се свържете с нас](https://fastcomments.com/auth/my-account/help).

### Липсващи коментари на същия URL с различен hash bang

По подразбиране FastComments ще използва URL на страницата за "кошницата", където се съхраняват коментарите. Ако вашите URL-и включват `#hashbangs` и тези `#hashbangs` не трябва да са част от идентификатора, който идентифицира нишка за коментари, можем просто да игнорираме стойността на hash bang, например:

[inline-code-attrs-start title = 'Пример за игнориране на Hash Bangs'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Имайте предвид, че след тази промяна ще трябва да се извърши миграция за съществуващите коментари. [За това се свържете с нас.](https://fastcomments.com/auth/my-account/help)

### URL параметри на заявката влияят на уиджета

По подразбиране FastComments ще използва URL на страницата за "кошницата", където се съхраняват коментарите. Ако вашите URL-и включват параметри на заявката, които не трябва да са част от идентификатора, който идентифицира нишка за коментари, можем просто да ги игнорираме, например:

[inline-code-attrs-start title = 'Игнориране на параметри на заявката'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Имайте предвид, че след тази промяна ще трябва да се извърши миграция за съществуващите коментари. [За това се свържете с нас.](https://fastcomments.com/auth/my-account/help)

### Не получавате имейли

Във FastComments влагаме много усилия да гарантираме, че доставката на нашите имейли е възможно най-надеждна. Въпреки това някои доставчици на имейли са известни с трудната надеждна доставка. Проверете папката за спам за съобщения от fastcomments.com.

Ако [се свържете с нас](https://fastcomments.com/auth/my-account/help), обикновено можем да предоставим повече информация за това защо може да не виждате имейли от нас.
