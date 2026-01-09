[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

По подразумјевању, функције форматирања у FastComments се обављају додавањем видљивих ознака као што су `<b></b>` око вашег текста. Кликом на траку са алаткама
или коришћењем пречица то ради уместо вас. Међутим, неке заједнице можда желе да се опредјеле за коришћење форматирања без видљивих ознака. Ово се зове омогућавање
WYSIWYG (оно што видите је оно што добијете) уређивача. Овај уређивач изгледа сасвим исто као подразумевани, осим што учитаје неки
додатни код који омогућава корисницима да свој текст подебљају, подвлаче и слично без видљивих ознака.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

Ово се може урадити и без кода. На страници за прилагођавање виџета, погледајте опцију "Омогући напредно форматирање".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]

---