[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

Подразумевано, функционалност форматирања у FastComments се реализује додавањем видљивих anchor ознака као `<b></b>` око вашег текста. Клик на траку са алаткама или коришћење пречица то ради за вас. Међутим, неке заједнице могу желети да користе форматирање без anchor ознака. Ово се назива омогућавање WYSIWYG (what you see is what you get) уређивача. Овај уређивач изгледа потпуно исто као подразумевани, осим што учитава додатни код који омогућава корисницима да подебљају, подвлаче и сл. свој текст без видљивих anchor ознака.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Омогућавање WYSIWYG уређивања'; code-example-end]

Ово се такође може урадити без кода. На страници за прилагођавање виџета, погледајте опцију "Enable Advanced Formatting" option.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; alt='Страница за прилагођавање виџета са означеним пољетом Enable Advanced Formatting за укључивање WYSIWYG уређивача'; title='Омогући WYSIWYG' app-screenshot-end]

---