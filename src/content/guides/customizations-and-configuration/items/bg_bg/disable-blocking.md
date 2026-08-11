[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments позволява на потребителите да блокират други потребители. Блокирането на потребител ще доведе до маскиране на техните коментари, предотвратява известията между потребителите и т.н.

Може да бъде желателно да се изключи тази функционалност. Това може да се направи по следния начин:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Изключване на блокирането'; code-example-end]

Това също може да се направи без код, което също така позволява правилна валидиране от страна на сървъра, чрез потребителския интерфейс за персонализиране на уиджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; alt='Опция за изключване на блокирането в потребителския интерфейс за персонализиране на уиджета, която спира потребителите да блокират един друг'; title='Изключване на блокирането' app-screenshot-end]