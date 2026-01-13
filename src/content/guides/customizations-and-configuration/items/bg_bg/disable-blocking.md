[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments позволява на потребителите да блокират други потребители. Блокирането на потребител ще доведе до маскиране на техните коментари
и ще предотврати уведомления между потребителите и т.н.

Може да е желателно да се деактивира тази функционалност. Това може да стане по следния начин:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

Това може да стане и без код, което също активира правилна валидация от страна на сървъра, чрез интерфейса за персонализиране на уиджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]

---