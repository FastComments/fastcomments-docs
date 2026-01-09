### Podrška za tamni način

Image Chat uključuje ugrađenu podršku za tamni način. Kada postavite `hasDarkBackground: true` u svojoj konfiguraciji, prozori chata i UI elementi automatski se prilagođavaju kako bi dobro funkcionirali na tamnim pozadinama.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

Stilizacija tamnog načina primjenjuje se na prozore chata, kvadratiće markera i sve interaktivne elemente. Ako vaša stranica ima prekidač za tamni način, možete ponovo inicijalizirati widget kada se način promijeni, ili koristiti pristup s klasom body opisan u nastavku.

### Dinamički tamni način

Ako je tamni način na vašoj stranici kontroliran dodavanjem klase `.dark` na element body, Image Chat UI će automatski poštovati to bez potrebe za ponovnom inicijalizacijom. Stilovi widgeta su dizajnirani da reagiraju na prisutnost te klase.

```css
/* Vaš CSS za tamni način */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
```

### Prilagođavanje stila pomoću CSS-a

Možete prilagoditi izgled markera, prozora chata i drugih elemenata pomoću CSS-a. Widget dodaje posebne klase koje možete ciljati u svom stylesheetu.

Chat kvadratići i prozori koriste FastComments sustav stiliziranja komentarskih oblačića, pa će sve prilagodbe koje ste primijenili na standardni widget za komentiranje također utjecati na Image Chat.

### Veličina chat kvadratića

Opcija `chatSquarePercentage` kontrolira veličinu klikabilnih markera. Zadana vrijednost je 5% širine slike:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 7  // Veće, vidljivije kvadratiće
});
```

Manje vrijednosti stvaraju diskretnije markere koji se bolje uklapaju u sliku. Veće vrijednosti čine markere istaknutijima i lakšima za klik, posebno na mobilnim uređajima ili iz pristupačnih razloga.

### Ponašanje na mobilnim uređajima

Na ekranima širim manje od 768px, Image Chat se automatski prebacuje na layout optimiziran za mobilne uređaje. Prozori chata se prikazuju preko cijelog zaslona umjesto da plutaju pored markera, što poboljšava upotrebljivost na malim ekranima.

Markeri ostaju vidljivi na svojim responzivnim pozicijama na slici. Korisnici mogu dodirnuti bilo koji marker da otvore fullscreen sučelje chata. Ovo ponašanje je ugrađeno i ne zahtijeva konfiguraciju.

### Izgled prozora chata

Prozori chata imaju širinu od 300px na desktopu s 16px strelicom koja pokazuje na marker. Prozori se automatski pozicioniraju na temelju dostupnog prostora u viewportu, koristeći klase pozicioniranja poput `to-right`, `to-left`, `to-top` i `to-bottom`.

Možete dodati prilagođeni CSS za podešavanje boja, fontova, razmaka ili drugih vizualnih svojstava ovih prozora. Prozori chata koriste istu strukturu komponenti kao standardni FastComments widget, pa nasljeđuju sve globalne prilagodbe koje ste primijenili.

### Lenja inicijalizacija

Prozori chata se inicijaliziraju pri hoveru za desktop korisnike ili odmah kada su stvoreni. Time se smanjuje početno opterećenje učitavanja jer se sučelje chata renderira samo kada korisnici zapravo komuniciraju s markerom.

Lenja inicijalizacija se odvija transparentno. Korisnici ne primjećuju kašnjenje, ali pregledniku nije potrebno renderirati desetke skrivenih prozora chata ako imate mnogo markera na slici.

### Lokalizacija

Image Chat podržava sve iste opcije lokalizacije kao standardni FastComments widget. Postavite opciju `locale` za prikaz UI teksta na različitim jezicima:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'fr'  // Francuski
});
```

FastComments podržava desetke jezika. Postavka lokaliteta utječe na sav UI tekst uključujući upite, gumbe i tekst mjesta za unos.

### Naslijeđene opcije prilagodbe

Budući da Image Chat proširuje standardni widget za komentiranje, nasljeđuje sve opcije prilagodbe iz osnovnog widgeta. To uključuje prilagođene CSS klase, prilagođene prijevode, prilagodbu avatara, formatiranje datuma i mnogo više.

Pogledajte glavnu FastComments dokumentaciju o prilagodbi za potpuni popis dostupnih opcija prilagodbe.

### Rad s prilagođenim fontovima

Ako vaša stranica koristi prilagođene fontove, Image Chat UI će naslijediti te fontove iz CSS-a vaše stranice. Prozori chata renderiraju unutar DOM-a vaše stranice i poštuju postojeća tipografska podešavanja.

Za najbolje rezultate, osigurajte da su vaši prilagođeni fontovi učitani prije inicijalizacije Image Chat-a, ili prihvatite da može doći do kratkog treptaja nestiliziranog teksta dok se fontovi učitavaju.

### Vizualni dizajn markera

Kvadratni marker ima suptilan vizualni dizajn koji ga čini uočljivim bez preplavljivanja slike. Možete prilagoditi njihov izgled pomoću CSS-a ako želite drugačiji vizualni pristup.

Markeri sadrže hover stanja koja daju povratnu informaciju kada korisnici pomaknu miš preko njih. Na uređajima na dodir, interakcija dodirom daje neposrednu povratnu informaciju otvaranjem prozora chata.