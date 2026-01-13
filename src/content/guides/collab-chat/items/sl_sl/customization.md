### Podpora temnemu načinu

### Dinamičen temni način

Če je temni način vaše strani nadzorovan z dodajanjem razreda `.dark` elementu body, bo uporabniški vmesnik Collab Chata to samodejno upošteval brez ponovne inicializacije. Slogi vtičnika so zasnovani tako, da reagirajo na prisotnost tega razreda.

[inline-code-attrs-start title = 'Primer CSS za temni način'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/* Vaš CSS za temni način */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
[inline-code-end]

### Prilagajanje videza s CSS

Videz poudarkov, klepetalnih oken in drugih elementov lahko prilagodite s CSS. Vtičnik doda posebne razrede, na katere lahko ciljate v svojem slogovnem listu.

Poudarki besedila uporabljajo sistem slogov mehurčkov komentarjev FastComments, zato bodo vse prilagoditve, ki ste jih uporabili za standardni komentarni vtičnik, vplivale tudi na Collab Chat.

### Prilagoditev zgornje vrstice

Zgornja vrstica prikazuje število uporabnikov na spletu in število razprav. Njeno pozicijo lahko prilagodite tako, da podate lastni element kot `topBarTarget`:

[inline-code-attrs-start title = 'Prilagojena lokacija zgornje vrstice'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('my-custom-header')
});
[inline-code-end]

Ali ga popolnoma onemogočite tako, da ga nastavite na `null`:

[inline-code-attrs-start title = 'Onemogoči zgornjo vrstico'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});
[inline-code-end]

### Obnašanje na mobilnih napravah

Na zaslonih širših manj kot 768px se Collab Chat samodejno preklopi v mobilno optimizirano postavitev. Okna klepeta se prikažejo na celotnem zaslonu namesto lebdeče ob besedilu, in zamik pri izbiri je odstranjen za bolj takojšnje interakcije.

To vedenje je vgrajeno in ne zahteva nikakršne konfiguracije. Vtičnik samodejno zazna velikost zaslona in se temu primerno prilagodi.

### Videz okna klepeta

Okna klepeta so na namizju široka 410px z 16px puščico, ki kaže na izpostavljeno besedilo. Okna se samodejno postavijo glede na razpoložljiv prostor v oknu brskalnika, z uporabo pozicionirnih razredov, kot so `to-right`, `to-left`, `to-top` in `to-bottom`.

Lahko dodate lasten CSS za prilagoditev barv, pisav, razmikov ali drugih vizualnih lastnosti teh oken. Okna klepeta uporabljajo isto strukturo komponent kot standardni vtičnik FastComments, zato podedujejo vse globalne prilagoditve, ki ste jih uporabili.

### Lokalizacija

Collab Chat podpira enake možnosti lokalizacije kot standardni vtičnik FastComments. Nastavite možnost `locale`, da se besedilo UI prikaže v različnih jezikih:

[inline-code-attrs-start title = 'Nastavitev lokalizacije'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // španščina
});
[inline-code-end]

FastComments podpira na desetine jezikov. Nastavitev lokalizacije vpliva na vse besedilo uporabniškega vmesnika, vključno z navodili, gumbi in besedilom nadomestka.

### Podedovane možnosti prilagoditve

Ker Collab Chat razširja standardni komentarni vtičnik, podeduje vse možnosti prilagoditve iz osnovnega vtičnika. To vključuje prilagojene CSS razrede, prilagojene prevode, prilagoditve avatarjev, oblikovanje datumov in še veliko več.

Oglejte si glavno dokumentacijo o prilagoditvah FastComments za celoten seznam razpoložljivih možnosti prilagoditve.

### Delo z lastnimi pisavami

Če vaša stran uporablja lastne pisave, jih uporabniški vmesnik Collab Chata podeduje iz CSS vaše strani. Morda boste morali ustvariti pravilo za prilagoditev vtičnika in `@import` vse pisave v prilagojenem CSS znotraj tega pravila, če želite, da lebdeče okno klepeta uporablja enake pisave.