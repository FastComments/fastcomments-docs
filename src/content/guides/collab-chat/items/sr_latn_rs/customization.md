### Podrška za tamni režim

### Dinamički tamni režim

Ako se tamni režim vašeg sajta kontroliše dodavanjem `.dark` klase na body element, Collab Chat UI će to automatski poštovati bez potrebe za ponovnom inicijalizacijom. Stilovi vidžeta dizajnirani su da reaguju na prisustvo ove klase.

[inline-code-attrs-start title = 'Primer CSS-a za tamni režim'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/* Your dark mode CSS */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
[inline-code-end]

### Prilagođeni stilovi pomoću CSS-a

Možete prilagoditi izgled isticanja teksta, prozora za ćaskanje i drugih elemenata pomoću CSS-a. Vidžet dodaje specifične klase koje možete ciljati u svom stilskom listu.

Isticanje teksta koristi sistem stilizovanja komentara FastComments, tako da će sve prilagodbe koje ste primenili na standardni vidžet za komentare uticati i na Collab Chat.

### Prilagođavanje gornje trake

Gornja traka prikazuje broj korisnika na mreži i broj diskusija. Možete prilagoditi njenu poziciju tako što ćete obezbediti prilagođeni element kao `topBarTarget`:

[inline-code-attrs-start title = 'Prilagođena lokacija gornje trake'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('my-custom-header')
});
[inline-code-end]

Ili je potpuno onemogućite postavljanjem na `null`:

[inline-code-attrs-start title = 'Onemogućavanje gornje trake'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});
[inline-code-end]

### Ponašanje na mobilnim uređajima

Na ekranima širine manje od 768px, Collab Chat automatski prelazi na mobilni optimizovani raspored. Prozori za ćaskanje se pojavljuju preko celog ekrana umesto da lebde pored teksta, a kašnjenje pri selekciji se uklanja radi brže interakcije.

Ovo ponašanje je ugrađeno i ne zahteva nikakvu konfiguraciju. Vidžet automatski detektuje veličinu ekrana i prilagođava se u skladu s tim.

### Izgled prozora za ćaskanje

Prozori za ćaskanje su široki 410px na desktopu sa strelicom od 16px koja pokazuje na istaknuti tekst. Prozori se automatski pozicioniraju na osnovu dostupnog prostora u vidnom polju, koristeći klase pozicioniranja poput `to-right`, `to-left`, `to-top` i `to-bottom`.

Možete dodati prilagođeni CSS da biste prilagodili boje, fontove, razmake ili druge vizuelne osobine ovih prozora. Prozori za ćaskanje koriste istu strukturu komponenti kao standardni FastComments vidžet, tako da nasleđuju sve globalne prilagodbe koje ste primenili.

### Lokalizacija

Collab Chat podržava sve iste opcije lokalizacije kao standardni FastComments vidžet. Postavite opciju `locale` da biste prikazali tekst UI na različitim jezicima:

[inline-code-attrs-start title = 'Podešavanje lokalizacije'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spanish
});
[inline-code-end]

FastComments podržava desetine jezika. Podešavanje lokalizacije utiče na sav tekst UI, uključujući upite, dugmad i tekst rezervisanog mesta.

### Nasleđene opcije prilagođavanja

Pošto Collab Chat proširuje standardni vidžet za komentare, nasleđuje sve opcije prilagođavanja osnovnog vidžeta. To uključuje prilagođene CSS klase, prilagođene prevode, prilagođavanje avatara, formatiranje datuma i mnogo više.

Pogledajte glavnu FastComments dokumentaciju za prilagođavanje za kompletnu listu dostupnih opcija prilagođavanja.

### Rad sa prilagođenim fontovima

Ako vaš sajt koristi prilagođene fontove, Collab Chat UI će naslediti te fontove iz CSS-a vaše stranice. Možda ćete morati da kreirate pravilo prilagođavanja vidžeta i da `@import`-ujete fontove u prilagođeni CSS u tom pravilu ako želite da lebdeći prozor za ćaskanje koristi iste fontove.