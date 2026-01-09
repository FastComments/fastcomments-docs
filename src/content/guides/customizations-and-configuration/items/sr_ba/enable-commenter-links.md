[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

По подразумевању, FastComments ће од корисника тражити само њихов коментар, корисничко име и е-пошту.

Међутим, у неким ситуацијама можда ћете желети да корисник остави везу ка свом блогу или веб-сајту.

Можемо омогућити приказ додатног поља за унос URL-а веб-сајта корисника постављањем флага **enableCommenterLinks** на true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

Када је наведени URL обезбеђен, налог корисника ће бити ажуриран и сва њихова корисничка имена на прошлим и будућим коментарима ће водити на тај URL.

Ово се може прилагодити без кода, на страници за прилагођавање виџета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]