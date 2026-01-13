---
[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

По подразумевању, FastComments ће од корисника тражити само њихов коментар, корисничко име и имејл.

Међутим, у неким ситуацијама можда ћете пожелети да корисник остави везу ка свом блогу или веб-сајту.

Можемо омогућити приказ додатног уносног поља за остављање URL адресе веб-сајта корисника постављањем флага **enableCommenterLinks** на true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Омогућавање линкова коментатора'; code-example-end]

Када је та URL адреса унесена, налог корисника ће бити ажуриран, и сва корисничка имена тог корисника на свим прошлим и будућим коментарима биће повезана са том URL адресом.

Ово се може прилагодити без кода, на страници за прилагођавање видгета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Омогућавање линкова коментатора' app-screenshot-end]

---