[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

По подразбиране функционалностите за форматиране в FastComments се извършват чрез добавяне на видими маркерни тагове като `<b></b>` около вашия текст. Кликването върху лентата с инструменти
или използването на клавишни комбинации прави това вместо вас. Въпреки това, някои общности може да искат да изберат използване на форматиране без видими маркерни тагове. Това се нарича активиране на WYSIWYG (каквото виждате, това получавате) редактора. Този редактор изглежда точно като стандартния, с изключение че зарежда допълнителен код, който позволява на потребителите да удебеляват, подчертават и т.н. своя текст без видими маркерни тагове.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

Това може да се направи и без код. В страницата за персонализиране на уиджета вижте опцията "Разреши разширено форматиране".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]