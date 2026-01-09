[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments ще показва значките на потребителите само върху техните коментари в рамките на нишката с коментари.

Въпреки това можем да показваме значките на потребителите до тяхното име над формата за коментар, като активираме тази функция в страницата за персонализиране на джаджата:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

Това ще показва значките на потребителя до неговото име в областта на горната лента, правейки техните постижения и статус по-видими, когато пишат коментар.

Обърнете внимание, че тази функция трябва да бъде активирана в UI за персонализиране на джаджата, за да работи. По желание можете да зададете флага **showBadgesInTopBar** на false във вашата конфигурация на кода, за да го деактивирате селективно, дори когато е включен на ниво сървър:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]