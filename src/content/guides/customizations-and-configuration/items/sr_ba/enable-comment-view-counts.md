[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

По подразумеваној вредности, FastComments не прати ко је прегледао сваки коментар нити пружа било какве статистике у вези с тим.

Међутим, ову функцију можемо омогућити, и тада ће систем почети да прати када се сваки корисник скролује до коментара.

Када се то догоди, бројач поред иконе ока који се приказује на сваком коментару биће увећан. Тај број се ажурира уживо и скраћује се у складу са локалом корисника.

Ово можемо омогућити подешавањем ознаке **enableViewCounts** на true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

Ово се може прилагодити без кода, на страници за прилагођавање виџета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; title='Enabling Comment View Counts' app-screenshot-end]

Пратимо user id* који је прегледао коментар, тако да ако погледате коментар поново, број се неће повећати. Ако поново погледате коментар након двије године, број ће се поново повећати.

- *Напомена: или anon session id, или the user's IP као хеширана вредност.

---