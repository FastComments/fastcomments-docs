Ko ljudem omogočamo dodajanje vsebine na spletno stran in nato prikazovanje te vsebine na številnih različnih vrstah naprav, obstaja več vidikov varnosti.

### Preprečevanje zlorabe oblikovanja

Ljudje lahko pišejo vsebino, ki je namerno vizualno moteča in zmanjšuje vrednost razprav z zlorabo oblikovanja besedila.

FastComments dela več stvari za preprečevanje zlorabe v zvezi z oblikovanjem:

- Velike ponavljajoče se zaporedne prelome vrstic se strni.
- Ne prikazujemo naslovov (postanejo običajno besedilo).
- Ne dovoljujemo CSS-ja ali prilagojenih barv.

### Preprečevanje izkoriščanja

Izkoriščanja se lahko ustvarijo v sistemih, ki prikazujejo HTML. FastComments dela več stvari za preprečevanje tega:

- Dovoljujemo samo izrecno določen nabor oznak HTML.
- Dovoljujemo samo izrecno določen nabor atributov oznak HTML.
- Čistimo in saniramo vse vnose.
  - To se izvaja prek knjižnic [DOMPurify](https://www.npmjs.com/package/dompurify) in [sanitizeHtml](https://www.npmjs.com/package/sanitize-html).
  - Te knjižnice smo izbrali, ker so dobro preizkušene (z več kot 4 in 1 milijonom prenosov na teden).

To pomeni, da uporabniki ne morejo početi stvari, kot je pisanje oznak `<script>` ali `<style>`, ali poskušati dodati skripte tipa `onload=alert()` slikam ali drugi vsebini.

Oznake HTML, ki jih dovoljujemo, so naslednje:

- `<b>`
- `<em>`
- `<u>`
- `<i>`
- `<strike>`
- `<pre>`
- `<span>`
- `<code>`
- `<img>`
- `<a>`
- `<strong>`
- `<ul>`
- `<ol>`
- `<li>`
- `<br>`
