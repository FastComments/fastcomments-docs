[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

За удостоверяване FastComments разчита на това бисквитките от трети страни да са разрешени в браузъра ви. Без тях, потребителите винаги ще трябва да
оставят своя имейл, за да коментират (освен ако полето за имейл не е скрито), и техните коментари винаги ще се показват като неверифицирани (по подразбиране).

За да заобиколите това, можете да активирате заобикалянето на бисквитките от трети страни. 

Когато тази настройка е активирана, ще се появи малък изскачащ прозорец, който показва съобщение, че потребителят се вписва. Този изскачащ прозорец
се появява всеки път, когато потребителят взаимодейства с компонента за коментари; например, ако остави коментар.

Можем да направим това в кода, като зададем флага **enableThirdPartyCookieBypass** на true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

Можем също да го настроим чрез интерфейса за персонализиране на уиджета, под `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]

---