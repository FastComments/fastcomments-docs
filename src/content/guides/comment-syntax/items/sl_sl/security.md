There are multiple aspects to security when letting people add content to a website
and then rendering that content on many different types of devices.

### Preprečevanje zlorabe oblikovanja

Ljudje lahko napišejo vsebino, ki je namerno vizualno moteča in zmanjšuje vrednost razprav z zlorabo oblikovanja besedila.

FastComments izvaja več ukrepov za preprečevanje zlorab pri oblikovanju:

- Velike ponavljajoče se zaporedne prelome vrstic strnemo.
- Naslovov ne upodabljamo (postanejo običajno besedilo).
- Ne dovoljujemo CSS-a ali prilagojenih barv.

### Preprečevanje izkoriščanj

V sistemih, ki upodabljajo HTML, se lahko pojavijo izkoriščanja. FastComments naredi več stvari, da to prepreči:

- Dovolimo le izrecno določeno množico HTML oznak.
- Dovolimo le izrecno določen nabor atributov HTML oznak.
- Vse vnose prečistimo in sanitiziramo.
  - To se izvaja z uporabo knjižnic [DOMPurify](https://www.npmjs.com/package/dompurify) in [sanitizeHtml](https://www.npmjs.com/package/sanitize-html).
  - Te knjižnice smo izbrali, ker so dobro preizkušene (s več kot 4 oziroma 1 milijonom prenosov na teden).

To pomeni, da uporabniki ne morejo narediti stvari, kot so pisanje `<script>` ali `<style>` oznak, ali poskusi dodajanja skript, kot je `onload=alert()`, slikam ali drugi vsebini.

The HTML tags we allow are as follows:

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

Oznaka `<iframe>` privzeto ni dovoljena. Če vklopite Dovoli vdelave medijev, so iframes tudi dovoljeni, vendar le, kadar je njihov vir eden izmed vgrajenih seznamov zaupanja vrednih ponudnikov (kot so YouTube, Vimeo, SoundCloud in Spotify) ali gostiteljsko ime, ki ste ga izrecno dodali. Iframe iz katerega koli drugega vira se odstranijo.