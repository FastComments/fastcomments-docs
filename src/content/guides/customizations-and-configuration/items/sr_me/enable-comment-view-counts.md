[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

По подразумевању, FastComments не прати ко је прегледао сваки коментар нити пружа било какву статистику у вези с тим.

Међутим, можемо омогућити ову функцију, и онда ће систем почети да прати када се сваки корисник скролује до коментара.

Када се то догоди, бројач поред иконе ока који се приказује на сваком коментару ће се повећати. Број се ажурира у реалном времену и скраћује се у складу са локалитетом корисника.

Ово можемо омогућити подешавањем флага **enableViewCounts** на true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

Ово се може прилагодити без кода, на страници за прилагођавање виџета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; title='Enabling Comment View Counts' app-screenshot-end]

Праћамо user id* који је прегледао коментар, тако да ако поново погледате коментар он се неће повећати. Ако погледате коментар поново након две године, број ће се поново повећати.

- *Напомена: или anon session id, или корисников IP као хешована вриједност.

---