[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

По подразумеваној поставци, FastComments ће тражити од корисника само њихов коментар, корисничко име и е-пошту.

Међутим, у неким ситуацијама можда ћете пожелети да корисник остави везу до свог блога или веб-сајта.

Можемо омогућити приказ додатног поља за унос у које ће корисник оставити URL свог сајта тако што ћемо поставити флаг **enableCommenterLinks** на true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

Када је тај URL наведен, налог корисника ће бити ажуриран и сва њихова корисничка имена на свим претходним и будућим коментарима ће водити на тај URL.

Ово се може прилагодити без кода, на страници за прилагођавање виџета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]

---