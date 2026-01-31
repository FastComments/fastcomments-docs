Ось кілька симптомів, з якими ми часто стикаємося, та поширені рішення. 

### Повідомлення "This is a demo"

Це відображається, коли ви скопіювали код віджета з нашої головної сторінки, яка використовує демонстраційний тенант. Щоб використовувати ваш тенант, скопіюйте код віджета з [тут](https://fastcomments.com/auth/my-account/get-acct-code).

### Помилка "FastComments cannot load on this domain"

FastComments повинен знати, які домени належать вам, щоб автентифікувати запити, пов'язані з вашим обліковим записом. [Перегляньте нашу документацію](/guide-multiple-sites.html#add-domains-to-account), щоб дізнатися, як вирішити цю помилку (просто додайте точний субдомен + домен до вашого облікового запису).

Зверніть увагу, що це має відбуватися тільки після завершення пробного періоду. Під час пробного періоду будь-які запити з нових доменів автоматично додаються до вашого облікового запису.

### Перенесені коментарі не відображаються для індивідуальних установок

Зазвичай це трапляється, коли імпортовані коментарі прив'язані до `Page ID`, а ви передаєте URL (або нічого не передаєте, у такому випадку за замовчуванням використовується URL сторінки).

Ви можете відлагодити це, [експортувавши свої коментарі](https://fastcomments.com/auth/my-account/manage-data/export) і переглянувши стовпець `URL ID` (зараз стовпець `B`).

Переконайтеся, що значення, які ви бачите в стовпці `URL ID`, збігаються зі значеннями, які ви передаєте в конфігурацію віджета як параметр `urlId`.

Для детальнішого пояснення прочитайте нашу документацію [How Comments are Tied to Pages and Articles](/guide-customizations-and-configuration.html#url-id).

Якщо нічого не допомагає, [зв'яжіться з нами](https://fastcomments.com/auth/my-account/help).

### Віджет коментарів не відображається

Якщо віджет коментарів не відображається, перевірте консоль розробника Chrome на наявність помилок.

У більшості випадків при неправильній конфігурації віджет коментарів принаймні покаже помилку на сторінці, якщо його вдасться завантажити. Якщо нічого не видно, це зазвичай вказує на помилку сценарію.

### Бажана конфігурація не працює як очікувалося

Спробуйте наше [розширення Chrome](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US), щоб побачити, яку конфігурацію передають віджету коментарів. Якщо все інше не допомагає, зробіть скриншот того, що показує розширення Chrome, і [зв'яжіться з нами](https://fastcomments.com/auth/my-account/help).

### Коментарі відсутні на тій же URL з різними hash bang

За замовчуванням FastComments використовує URL сторінки як «сховище», де зберігаються коментарі. Якщо ваші URL містять `#hashbangs`, і ці `#hashbangs` не повинні бути частиною ідентифікатора, що визначає потік коментарів, ми можемо просто ігнорувати значення hash bang, наприклад:

[inline-code-attrs-start title = 'Приклад ігнорування хешбангів'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Зверніть увагу, що після внесення цієї зміни для існуючих коментарів доведеться виконати міграцію. [Для цього зв'яжіться з нами.](https://fastcomments.com/auth/my-account/help)

### Параметри запиту URL, що впливають на віджет

За замовчуванням FastComments використовує URL сторінки як «сховище», де зберігаються коментарі. Якщо ваші URL містять параметри запиту, які не повинні бути частиною ідентифікатора, що визначає потік коментарів, ми можемо просто ігнорувати їх, наприклад:

[inline-code-attrs-start title = 'Приклад ігнорування параметрів запиту'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Зверніть увагу, що після внесення цієї зміни для існуючих коментарів доведеться виконати міграцію. [Для цього зв'яжіться з нами.](https://fastcomments.com/auth/my-account/help)

### Не отримуєте електронні листи

У FastComments ми докладаємо багато зусиль, щоб забезпечити максимально надійну доставку електронних листів. Однак деякі поштові провайдери відомі тим, що до них важко доставляти повідомлення надійно. Перевірте папку спаму на наявність повідомлень від fastcomments.com.

Якщо ви [зв'яжетеся з нами](https://fastcomments.com/auth/my-account/help), ми зазвичай можемо надати більше інформації про те, чому ви можете не отримувати листи від нас.

---