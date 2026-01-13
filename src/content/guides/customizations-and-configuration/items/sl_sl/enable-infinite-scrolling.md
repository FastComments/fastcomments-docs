[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

Privzeto se pripomoček FastComments navpično prilagodi, da ustreza vsem vidnim komentarjem. Paginacija je dosežena z gumbom "Prikaži naslednje" na koncu trenutne strani, saj ugotavljamo, da je to interakcija, ki je za večino uporabnikov najbolj prijetna.

Vendar obstajajo primeri, kjer je bolj zaželeno neskončno pomikanje. Na primer, to funkcijo uporabljamo v našem izdelku Stream Chat.

Gumbe "Prikaži naslednje" lahko skrijemo in preklopimo na neskončno pomikanje tako, da zastavico **enableInfiniteScrolling** nastavite na true:

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

To zahteva tudi dodajanje prilagojenega CSS-a. Dodajte prilagojen CSS za selektor `.comments`, da omogočite pomikanje, na primer:

[inline-code-attrs-start title = 'Omogoči neskončno pomikanje'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

Popoln delujoč primer bi bil:

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

V zgornjem primeru uporabljamo lastnost `customCSS`, vendar je zaradi razlogov zmogljivosti priporočljivo namesto tega uporabiti vmesnik za konfiguracijo pripomočka (Widget Configuration UI). [Oglejte si dokumentacijo za Custom CSS.](/guide-customizations-and-configuration.html#custom-css)

---