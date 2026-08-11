[related-parameter-start name = 'enableSearch'; type = 'boolean'; related-parameter-end]

По подразбиране в уиджетa за коментари не се показва поле за търсене.

Въпреки това можем да го включим, като зададем флага **enableSearch** на true:

[code-example-start config = {enableSearch: true}; linesToHighlight = [6]; title = 'Активиране на търсене'; code-example-end]

Това може да се направи и без код. На страницата за персонализиране на уиджета, вижте опцията „Enable Search Box“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-search-label']; selector = '.enable-search-label'; alt='Страница за персонализиране на уиджета с отметка в полето „Enable Search Box“, за да се покаже поле за търсене в уиджета'; title='Включи полето за търсене' app-screenshot-end]