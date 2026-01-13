[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

Ko pošiljate obvestilna e‑poštna sporočila ali prikazujete komentarje v uporabniških vmesnikih, kot je stran za moderacijo, je koristno, da lahko povežete
iz komentarja na stran, na kateri se nahaja.

Če URL ID ni vedno dejanski ID, moramo URL shraniti nekje drugje. Za to je namenjena lastnost "url", definirana takole.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

Pogosta uporaba je vezava nitke komentarjev na identifikator, kot je članek, in nato povezava nazaj na določeno stran, na primer:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

URL ni očiščen običajnih marketinških parametrov. Privzeto se shrani URL, kakršen je trenutni URL strani.