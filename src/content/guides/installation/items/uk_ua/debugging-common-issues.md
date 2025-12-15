Ось деякі симптоми, з якими ми часто стикаємося, та типові рішення.

### Повідомлення "This is a demo"

Це відображається, коли ви скопіювали код віджета з нашої головної сторінки, яка використовує наш демо-тенант. Щоб використовувати ваш тенант, скопіюйте код віджета [звідси](https://fastcomments.com/auth/my-account/get-acct-code).

### Помилка "FastComments cannot load on this domain"

FastComments повинен знати, які домени належать вам, для автентифікації запитів, пов'язаних з вашим обліковим записом. [Ознайомтеся з нашою документацією](/guide-multiple-sites.html#add-domains-to-account), щоб дізнатися, як вирішити цю помилку (просто додайте точний піддомен + домен до вашого облікового запису).

Зверніть увагу, що це повинно відбуватися лише після закінчення пробного періоду. Під час пробного періоду будь-які запити з нових доменів автоматично додаються до вашого облікового запису.

### Перенесені коментарі не відображаються для користувацьких установок

Зазвичай це відбувається, коли імпортовані коментарі прив'язані до `Page ID`, а ви передаєте URL (або не передаєте значення, у цьому випадку за замовчуванням використовується URL сторінки).

Ви можете налагодити це, [експортувавши ваші коментарі](https://fastcomments.com/auth/my-account/manage-data/export) та переглянувши стовпець `URL ID` (наразі стовпець `B`).

Переконайтеся, що значення, які ви бачите у стовпці `URL ID`, збігаються зі значеннями, які ви передаєте у конфігурацію віджета як параметр `urlId`.

Для подальшого пояснення спробуйте прочитати нашу [документацію про те, як коментарі прив'язані до сторінок і статей](/guide-customizations-and-configuration.html#url-id).

Якщо нічого не допомагає, [зв'яжіться з нами](https://fastcomments.com/auth/my-account/help).

### Віджет коментарів не відображається

Якщо віджет коментарів не відображається, перевірте консоль розробника Chrome на наявність помилок.

При більшості помилок конфігурації віджет коментарів принаймні покаже помилку на сторінці, якщо він зможе завантажитися. Якщо нічого не видно, це зазвичай вказує на помилку скрипта.

### Бажана конфігурація не працює як очікувалося

Спробуйте наше [розширення Chrome](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US), щоб побачити, яка конфігурація передається віджету коментарів. Якщо нічого не допомагає, зробіть скріншот того, що показує розширення Chrome, і [зв'яжіться з нами](https://fastcomments.com/auth/my-account/help).

### Відсутні коментарі на одному URL з різними hash bang

За замовчуванням FastComments використовує URL сторінки як «контейнер», де зберігаються коментарі. Якщо ваші URL містять `#hashbangs`, і ці `#hashbangs` не повинні бути частиною ідентифікатора, який визначає гілку коментарів, ми можемо просто ігнорувати значення hash bang, наприклад:

[inline-code-attrs-start title = 'Приклад ігнорування Hash Bangs'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Зверніть увагу, що після внесення цієї зміни потрібно буде виконати міграцію для існуючих коментарів. [Для цього зв'яжіться з нами.](https://fastcomments.com/auth/my-account/help)

### Параметри запиту URL впливають на віджет

За замовчуванням FastComments використовує URL сторінки як «контейнер», де зберігаються коментарі. Якщо ваші URL містять параметри запиту, які не повинні бути частиною ідентифікатора, який визначає гілку коментарів, ми можемо просто ігнорувати їх, наприклад:

[inline-code-attrs-start title = 'Ігнорування параметрів запиту'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Зверніть увагу, що після внесення цієї зміни потрібно буде виконати міграцію для існуючих коментарів. [Для цього зв'яжіться з нами.](https://fastcomments.com/auth/my-account/help)

### Не отримуєте електронні листи

У FastComments ми докладаємо багато зусиль, щоб забезпечити максимально надійну доставку електронних листів. Однак деякі провайдери електронної пошти відомі тим, що до них важко надійно доставити листи. Перевірте папку спаму на наявність повідомлень від fastcomments.com.

Якщо ви [зв'яжетеся з нами](https://fastcomments.com/auth/my-account/help), ми зазвичай можемо надати більше інформації про те, чому ви можете не бачити листи від нас.
