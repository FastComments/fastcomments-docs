[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

По подразбиране, функционалностите за форматиране във FastComments се извършват чрез добавяне на видими етикети като `<b></b>` около вашия текст. Кликването върху лентата с инструменти
или използването на клавишни комбинации прави това за вас. Въпреки това, някои общности може да искат да използват форматиране без етикети. Това се нарича активиране на
WYSIWYG (what you see is what you get) редактора. Този редактор изглежда точно същият като стандартния, но зарежда някакъв
допълнителен код, който позволява на потребителите да удебеляват, подчертават и т.н. текста си без видими етикети.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

Това може да се направи и без код. На страницата за персонализиране на уиджета, вижте опцията "Enable Advanced Formatting".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; alt='Страница за персонализиране на уиджета с отметка Enable Advanced Formatting, за да се активира WYSIWYG редакторът'; title='Активиране на WYSIWYG' app-screenshot-end]