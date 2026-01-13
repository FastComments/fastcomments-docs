[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

По подразумевању, форматирање у FastComments се врши додавањем видљивих ознака као што су `<b></b>` око вашег текста. То се ради кликом на траку са алаткама
или коришћењем пречица. Међутим, неке заједнице можда желе да користе форматирање без видљивих ознака. Ово се назива омогућавањем
WYSIWYG (оно што видите је оно што добијате) уређивача. Тај уређивач изгледа управо као подразумевани, осим што учитава додатни код који омогућава корисницима да своје
текстове подебљају, подвлаче итд. без видљивих ознака.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

Ово се може урадити и без кода. На страници за прилагођавање виџета, погледајте опцију "Омогући напредно форматирање".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]