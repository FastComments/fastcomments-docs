---
[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

По подразумеваној поставци, форматирање у FastComments се обавља додавањем видљивих ознака као што су `<b></b>` око вашег текста. Кликом на траку са алаткама
или коришћењем пречица то ће бити урађено за вас. Међутим, неке заједнице можда желе да омогуће коришћење форматирања без видљивих ознака. Ово се назива омогућавањем
WYSIWYG (оно што видиш је оно што добијеш) уређивача. Тај уређивач изгледа потпуно исто као подразумевани, осим што учитава додатни
код који омогућава корисницима да подебљају, подвлаче и слично свој текст без видљивих ознака.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

Ово се такође може урадити без кода. На страници за прилагођавање виџета, погледајте опцију „Омогући напредно форматирање“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]

---