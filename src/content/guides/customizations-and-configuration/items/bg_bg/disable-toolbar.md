[related-parameter-start name = 'disableToolbar'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments ще показва лента с инструменти, когато се пише коментар, за да осигури преки пътища за форматиране на текста и качване на
изображения.

Тази лента с инструменти може да бъде деактивирана чрез код или чрез потребителския интерфейс за персонализиране.

[code-example-start config = {disableToolbar: true}; linesToHighlight = [6]; title = 'Disabling The Toolbar'; code-example-end]

Това може да се направи и без код. В страницата за персонализиране на уиджета вижте опцията "Изключване на лентата с инструменти за отговори".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-toolbar']; selector = '.disable-toolbar'; title='Disabling The Toolbar' app-screenshot-end]