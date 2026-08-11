[related-parameter-start name = 'noStyles'; type = 'boolean'; related-parameter-end]

За по-големи проекти за персонализирано стилизиране може да е желателно да започнете от нулата и изобщо да не използвате стандартното стилизиране.

Цялото стандартно стилизиране може да бъде премахнато, като зададете параметъра **noStyles** на true, както следва:

[code-example-start config = {noStyles: true}; linesToHighlight = [6]; title = 'Disabling All Default Styles'; code-example-end]

Това може да се персонализира без код, на страницата за персонализиране на уиджета, под „Разширени опции“:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.disable-all-default-styling']; selector = '.disable-all-default-styling'; alt='Отметка за деактивиране на цялото стандартно стилизиране, активирана под „Разширени опции“ на страницата за персонализиране на уиджета'; title='Деактивиране на цялото стандартно стилизиране' app-screenshot-end]