---
[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Подразумевано, FastComments ће тражити од корисника само њихов коментар, корисничко име и имејл.

Међутим, у неким ситуацијама можда желите да корисник остави везу ка свом блогу или веб локацији.

Можемо омогућити приказ додатног поља за унос где корисник може оставити URL своје веб странице постављањем заставице **enableCommenterLinks** на true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

Када се тај URL наведе, налог корисника ће бити ажуриран и све њихово корисничко име у свим прошлим и будућим коментарима ће бити везано за овај URL.

Ово се може прилагодити без кода, на страници за прилагођавање виџета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; alt='Widget customization page with the commenter links checkbox checked to add a website URL field to the comment form'; title='Enabling Commenter Links' app-screenshot-end]

---