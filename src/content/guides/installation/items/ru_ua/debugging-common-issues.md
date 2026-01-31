Здесь приведены некоторые симптомы, с которыми мы часто сталкиваемся, и распространённые решения. 

### Сообщение "This is a demo"

Это сообщение отображается, когда вы скопировали код виджета с нашей домашней страницы, которая использует наш демонстрационный tenant. Чтобы использовать ваш tenant, скопируйте код виджета [здесь](https://fastcomments.com/auth/my-account/get-acct-code).

### Ошибка "FastComments cannot load on this domain"

FastComments нужно знать, какими доменами вы владеете, чтобы аутентифицировать запросы, связанные с вашей учётной записью. [Ознакомьтесь с нашей документацией](/guide-multiple-sites.html#add-domains-to-account), чтобы узнать, как устранить эту ошибку (просто добавьте точный субдомен + домен в вашу учётную запись).

Обратите внимание, что это должно происходить только после окончания пробного периода. В течение пробного периода любые запросы с новых доменов автоматически добавляются в вашу учётную запись.

### Мигрированные комментарии не отображаются при пользовательских установках

Обычно это происходит, когда импортированные комментарии привязаны к `Page ID`, а вы передаёте URL (или не передаёте значение, в этом случае по умолчанию используется URL страницы).

Вы можете отладить это, [экспортировав ваши комментарии](https://fastcomments.com/auth/my-account/manage-data/export) и посмотрев столбец `URL ID` (в настоящее время столбец `B`).

Убедитесь, что значения, которые вы видите в столбце `URL ID`, совпадают со значениями, которые вы передаёте в конфигурацию виджета как параметр `urlId`.

Для дальнейших объяснений попробуйте прочитать нашу документацию [How Comments are Tied to Pages and Articles](/guide-customizations-and-configuration.html#url-id).

Если ничего не помогает, [свяжитесь с нами](https://fastcomments.com/auth/my-account/help).

### Виджет комментариев не отображается

Если виджет комментариев не отображается, проверьте консоль разработчика Chrome на наличие ошибок.

Для большинства некорректных настроек виджет комментариев по крайней мере покажет ошибку на странице, если он смог загрузиться. Ничего не видеть обычно означает ошибку в скрипте.

### Желаемая конфигурация работает не так, как ожидалось

Попробуйте наше [расширение для Chrome](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US), чтобы увидеть, какую конфигурацию получает виджет комментариев. Если ничего не помогает, сделайте снимок экрана того, что показывает расширение Chrome, и [свяжитесь с нами](https://fastcomments.com/auth/my-account/help).

### Комментарии отсутствуют на том же URL с разным Hash Bang

По умолчанию FastComments использует URL страницы для «bucket», где хранятся комментарии. Если ваши URL включают `#hashbangs`, и эти `#hashbangs` не должны быть частью идентификатора, который определяет поток комментариев, мы можем просто игнорировать значение hash bang, например:

[inline-code-attrs-start title = 'Игнорировать Hash Bangs — пример'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Обратите внимание, что после внесения этого изменения для существующих комментариев потребуется выполнить миграцию. [По этому вопросу свяжитесь с нами.](https://fastcomments.com/auth/my-account/help)

### Параметры запроса URL, влияющие на виджет

По умолчанию FastComments использует URL страницы для «bucket», где хранятся комментарии. Если ваши URL содержат параметры запроса, которые не должны быть частью идентификатора, определяющего поток комментариев, мы можем просто игнорировать их, например:

[inline-code-attrs-start title = 'Игнорировать параметры запроса'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Обратите внимание, что после внесения этого изменения для существующих комментариев потребуется выполнить миграцию. [По этому вопросу свяжитесь с нами.](https://fastcomments.com/auth/my-account/help)

### Не получаете письма

В FastComments мы прилагаем много усилий, чтобы доставка наших писем была максимально надёжной. Однако некоторые почтовые провайдеры известны своей ненадёжностью доставки. Проверьте папку спама на наличие сообщений от fastcomments.com.

Если вы [свяжетесь с нами](https://fastcomments.com/auth/my-account/help), мы обычно сможем дать больше информации о том, почему вы можете не получать от нас письма.

---