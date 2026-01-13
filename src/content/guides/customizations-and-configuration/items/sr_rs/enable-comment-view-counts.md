[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

По подразумеваној поставци, FastComments не прати ко је прегледао сваки коментар нити пружа било какву статистику у вези с тим.

Међутим, ову функцију можемо омогућити, и тада ће систем почети да прати када корисник скролује до коментара.

Када се то догоди, бројач поред иконе ока који се приказује на сваком коментару ће се повећати. Број се ажурира у реалном времену и скраћује се у складу са локалним подешавањима корисника.

Ову опцију можемо омогућити постављањем флага **enableViewCounts** на true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

Ово се може прилагодити без кода, на страници за прилагођавање видгета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; title='Enabling Comment View Counts' app-screenshot-end]

Пратимо и ID корисника* који је прегледао коментар, тако да ако поново погледате коментар, бројач се неће повећати. Ако поново погледате коментар након две године, број ће се повећати поново.

- *Напомена: или anon session id, или IP корисника као хеширана вредност.

---