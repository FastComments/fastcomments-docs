### Podrška za tamni način

### Dinamički tamni način

Ako je tamni način na vašoj stranici kontroliran dodavanjem klase `.dark` elementu body, Collab Chat UI automatski će to poštovati bez potrebe za ponovnim inicijaliziranjem. Stilovi widgeta dizajnirani su da reagiraju na prisutnost te klase.

[inline-code-attrs-start title = 'Dark Mode CSS Example'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/* Vaš CSS za tamni način */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
[inline-code-end]

### Prilagođeno stiliziranje putem CSS-a

Možete prilagoditi izgled isticanja, prozora za chat i drugih elemenata pomoću CSS-a. Widget dodaje specifične klase koje možete ciljati u svom stilskom listu.

Isticanje teksta koristi sustav stiliziranja komentarskih oblačića FastComments, pa će sve prilagodbe koje ste primijenili na standardni widget za komentare također utjecati na Collab Chat.

### Prilagodba gornje trake

Gornja traka prikazuje broj korisnika online i broj rasprava. Poziciju možete prilagoditi tako da navedete prilagođeni element kao `topBarTarget`:

[inline-code-attrs-start title = 'Custom Top Bar Location'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('my-custom-header')
});
[inline-code-end]

Ili je u potpunosti onemogućite postavljanjem na `null`:

[inline-code-attrs-start title = 'Disable Top Bar'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});
[inline-code-end]

### Ponašanje na mobilnim uređajima

Na zaslonima užim od 768px, Collab Chat se automatski prebacuje u izgled optimiziran za mobilne uređaje. Prozori chata pojavljuju se preko cijelog zaslona umjesto da lebde pored teksta, a kašnjenje pri odabiru je uklonjeno radi brže interakcije.

Ovo ponašanje je ugrađeno i ne zahtijeva konfiguraciju. Widget automatski detektira veličinu zaslona i prilagođava se u skladu s tim.

### Izgled prozora chata

Prozori chata na desktopu imaju širinu 410px uz strelicu od 16px koja pokazuje na istaknuti tekst. Prozori se automatski postavljaju ovisno o raspoloživom prostoru u viewportu, koristeći klase pozicioniranja poput `to-right`, `to-left`, `to-top`, i `to-bottom`.

Možete dodati prilagođeni CSS za podešavanje boja, fontova, razmaka ili drugih vizualnih svojstava tih prozora. Prozori chata koriste istu strukturu komponenata kao standardni FastComments widget, pa nasljeđuju sve globalne prilagodbe koje ste primijenili.

### Lokalizacija

Collab Chat podržava sve iste opcije lokalizacije kao standardni FastComments widget. Postavite opciju `locale` kako biste prikazali tekst sučelja na različitim jezicima:

[inline-code-attrs-start title = 'Set Locale'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // španjolski
});
[inline-code-end]

FastComments podržava desetke jezika. Postavka locale utječe na sav tekst sučelja, uključujući poruke, gumbe i tekst u poljima za unos (placeholder).

### Naslijeđene opcije prilagodbe

Budući da Collab Chat proširuje standardni widget za komentare, nasljeđuje sve opcije prilagodbe iz osnovnog widgeta. To uključuje prilagođene CSS klase, prilagođene prijevode, prilagodbu avatara, formatiranje datuma i još mnogo toga.

Pogledajte glavnu FastComments dokumentaciju o prilagodbi za potpuni popis dostupnih opcija prilagodbe.

### Rad s prilagođenim fontovima

Ako vaša stranica koristi prilagođene fontove, Collab Chat UI će naslijediti te fontove iz CSS-a vaše stranice. Možda ćete morati stvoriti pravilo za prilagodbu widgeta i `@import` bilo kojih fontova u prilagođenom CSS-u u tom pravilu ako
želite da plutajući prozor chata koristi iste fontove.