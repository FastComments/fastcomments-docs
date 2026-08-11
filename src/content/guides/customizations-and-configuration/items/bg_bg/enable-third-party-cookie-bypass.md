[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

За удостоверяване FastComments зависи от това, бисквитките от трети страни да са активирани в браузъра ви. Без тях потребителите винаги ще трябва да оставят имейл, за да коментират (освен ако полето за имейл е скрито), а коментарите им винаги ще се показват като непотвърдени (по подразбиране).

За да заобиколите това, можете да активирате заобикалянето на бисквитки от трети страни. 

Когато тази настройка е активирана, ще се появи малък изскачащ прозорец, който показва съобщение, че потребителят се вписва. Този прозорец се показва всеки път, когато потребителят взаимодейства с уиджета за коментари; например, ако остави коментар.

Можем да направим това в кода, като зададем флага **enableThirdPartyCookieBypass** на true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Включване на заобикаляне на бисквитки от трети страни'; code-example-end]

Можем също да настроим това чрез UI за персонализиране на уиджета, под `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; alt='Страница за персонализиране на уиджет с отметка за включен прозорец за бисквитки от трети страни'; title='Включване на заобикаляне на бисквитки от трети страни' app-screenshot-end]