FastComments nadzor dostopa deluje tako, da dodeli `Pages` in `Users` v želene skupine.

Skupina je preprosto identifikator v obliki niza, kot `GREEN` ali `abc-123`.

`Users` in `Pages` niso omejeni le na eno skupino. `Users` so omejeni na `100`, `Pages` pa na `1000` skupin. 

#### Dostop do neavtoriziranih strani

Če uporabnik poskuša dostopati do strani, do katere nima dostopa, bo videl sporočilo o napaki, kot je spodaj:

<div class="screenshot white-bg">
    <div class="title">Primer neuspešne avtorizacije</div>
    <img class="screenshot-image" src="/images/sso-unauthorized-message.png" alt="Primer neuspešne avtorizacije" />
</div>

Besedilo sporočila je mogoče prilagoditi.