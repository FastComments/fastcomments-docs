### Podpora temnega načina

Image Chat vključuje vgrajeno podporo temnega načina. Ko v svoji konfiguraciji nastavite `hasDarkBackground: true`, se klepetalna okna in elementi uporabniškega vmesnika samodejno prilagodijo, da dobro delujejo na temnih ozadjih.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

Stiliranje za temni način se uporablja za klepetalna okna, označevalne kvadrate in vse interaktivne elemente. Če ima vaše spletno mesto stikalo za temni način, lahko pripomoček znova inicializirate, ko se način spremeni, ali uporabite pristop z razredom body, opisan spodaj.

### Dinamični temni način

Če je temni način na vašem spletnem mestu nadzorovan z dodajanjem razreda `.dark` elementu body, bo Image Chat UI to samodejno upošteval brez potrebe po ponovni inicializaciji. Slogi pripomočka so zasnovani tako, da se odzovejo na prisotnost tega razreda.

```css
/* Vaš CSS za temni način */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
```

### Prilagajanje slogov s CSS

S CSS lahko prilagodite videz označevalcev, klepetalnih oken in drugih elementov. Pripomoček doda specifične razrede, na katere lahko ciljate v svoji datoteki s slogom.

Označevalni kvadrati in okna klepeta uporabljajo sistem za oblikovanje komentarnih oblačkov FastComments, zato bodo vse prilagoditve, ki ste jih uporabili za standardni pripomoček za komentiranje, vplivale tudi na Image Chat.

### Velikost označevalnih kvadratov

Možnost `chatSquarePercentage` nadzoruje velikost klikljivih označevalcev. Privzeto je 5% širine slike:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 7  // Večji, bolj vidni kvadrati
});
```

Manjše vrednosti ustvarijo bolj diskretne označevalce, ki se zlijejo s sliko. Večje vrednosti naredijo označevalce bolj izrazite in lažje za klik, še posebej na mobilnih napravah ali zaradi dostopnosti.

### Obnašanje na mobilnih napravah

Na zaslonih, ožjih od 768px, se Image Chat samodejno preklopi na postavitev, optimizirano za mobilne naprave. Klepetalna okna se prikažejo v celozaslonskem načinu namesto lebdenja ob označevalcih, kar zagotavlja boljšo uporabnost na majhnih zaslonih.

Označevalci ostanejo vidni na svojih odzivnih položajih na sliki. Uporabniki lahko tapnejo kateri koli označevalec za odpiranje celozaslonskega vmesnika klepeta. To vedenje je vgrajeno in ne zahteva dodatne konfiguracije.

### Videz klepetalnih oken

Klepetalna okna so na namizju široka 300px z 16px puščico, ki kaže na označevalec. Okna se samodejno pozicionirajo glede na razpoložljivi prostor v pogledu, z uporabo razredov za pozicioniranje, kot so `to-right`, `to-left`, `to-top` in `to-bottom`.

Lahko dodate lastni CSS za prilagoditev barv, pisav, razmikov ali drugih vizualnih lastnosti teh oken. Klepetalna okna uporabljajo isto strukturo komponent kot standardni pripomoček FastComments, zato dedujejo vse globalne prilagoditve, ki ste jih uporabili.

### Odložena inicializacija

Klepetalna okna se inicializirajo ob premiku miške za uporabnike na namizju ali takoj ob ustvarjanju. To zmanjša začetno obremenitev z nalaganjem, saj se vmesnik klepeta upodobi le, ko uporabniki dejansko sodelujejo z označevalcem.

Odložena inicializacija se zgodi prosojno. Uporabniki ne opazijo zamude, vendar brskalnik ne potrebuje upodabljati ducatov skritih klepetalnih oken, če imate na sliki veliko označevalcev.

### Lokalizacija

Image Chat podpira vse enake možnosti lokalizacije kot standardni pripomoček FastComments. Nastavite možnost `locale`, da prikažete besedilo vmesnika v različnih jezikih:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'fr'  // Francoščina
});
```

FastComments podpira desetine jezikov. Nastavitev locale vpliva na vse besedilo vmesnika, vključno z navodili, gumbi in nadomestnim besedilom.

### Podedovane možnosti prilagajanja

Ker Image Chat razširja standardni pripomoček za komentiranje, podeduje vse možnosti prilagajanja osnovnega pripomočka. To vključuje lastne CSS razrede, lastne prevode, prilagajanje avatarjev, oblikovanje datumov in še veliko več.

Oglejte si glavno dokumentacijo za prilagajanje FastComments za popoln seznam razpoložljivih možnosti prilagajanja.

### Delo s prilagojenimi pisavami

Če vaše spletno mesto uporablja prilagojene pisave, jih bo Image Chat UI podedoval iz CSS vaše strani. Klepetalna okna se upodabljajo znotraj DOM vaše strani in spoštujejo obstoječe tipografske nastavitve.

Za najboljše rezultate poskrbite, da bodo vaše prilagojene pisave naložene pred inicializacijo Image Chat, ali pa sprejmite, da se lahko med nalaganjem pisav pojavi kratek utrip nestilirane pisave.

### Vizualna zasnova označevalcev

Kvadratni označevalci imajo subtilno vizualno zasnovo, zaradi katere so opazni, ne da bi preplavili sliko. Njihov videz lahko prilagodite s CSS, če želite drugačno vizualno obdelavo.

Označevalci vključujejo stanja ob premiku miške, ki nudijo povratne informacije, ko uporabniki premaknejo miško nad njimi. Na napravah na dotik pa interakcija s tapom zagotovi takojšnje povratne informacije z odpiranjem klepetalnega okna.

---