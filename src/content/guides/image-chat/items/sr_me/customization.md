### Podrška za tamni režim

Image Chat ima ugrađenu podršku za tamni režim. Kada postavite `hasDarkBackground: true` u vašoj konfiguraciji, prozori za chat i UI elementi se automatski prilagođavaju da dobro funkcionišu na tamnim pozadinama.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

Stilizacija za tamni režim se primjenjuje na prozore chata, kvadrate markera i sve interaktivne elemente. Ako vaš sajt ima prekidač za tamni režim, možete ponovo inicijalizovati widget kada se režim promijeni, ili koristiti pristup sa klasom na body elementu opisan ispod.

### Dinamički tamni režim

Ako je tamni režim na vašem sajtu kontrolisan dodavanjem klase `.dark` na body element, Image Chat UI će automatski poštovati ovo bez potrebe za ponovnom inicijalizacijom. Stilovi widgeta su dizajnirani da reaguju na prisustvo ove klase.

```css
/* Vaš CSS za tamni režim */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
```

### Prilagođavanje stila pomoću CSS-a

Možete prilagoditi izgled markera, prozora chata i drugih elemenata koristeći CSS. Widget dodaje specifične klase koje možete ciljati u vašem stylesheet-u.

Kvadrati i prozori chata koriste FastComments sistem stilizovanja balončića za komentare, tako da će sve prilagodbe koje ste primijenili na standardni widget za komentare takođe uticati i na Image Chat.

### Veličina chat kvadrata

Opcija `chatSquarePercentage` kontroliše veličinu klikabilnih markera. Podrazumijevano je 5% širine slike:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 7  // Veći, vidljiviji kvadrati
});
```

Manje vrijednosti stvaraju diskretnije markere koji se bolje uklapaju u sliku. Veće vrijednosti čine markere istaknutijim i lakšim za klik, posebno na mobilnim uređajima ili radi pristupačnosti.

### Ponašanje na mobilnim uređajima

Na ekranima užim od 768px, Image Chat se automatski prebacuje na izgled optimizovan za mobilne uređaje. Prozori chata se pojavljuju preko cijelog ekrana umjesto da plutaju pored markera, što poboljšava upotrebljivost na malim ekranima.

Markeri ostaju vidljivi na svojim responzivnim pozicijama na slici. Korisnici mogu dodirnuti bilo koji marker da otvore interfejs chata preko cijelog ekrana. Ovo ponašanje je ugrađeno i ne zahtijeva nikakvu konfiguraciju.

### Izgled prozora chata

Prozori chata su široki 300px na desktopu sa strelicom od 16px koja pokazuje na marker. Prozori se pozicioniraju automatski na osnovu dostupnog prostora u viewport-u, koristeći klase pozicioniranja kao što su `to-right`, `to-left`, `to-top`, i `to-bottom`.

Možete dodati prilagođeni CSS da podesite boje, fontove, razmake ili druge vizuelne osobine ovih prozora. Prozori chata koriste istu komponentnu strukturu kao standardni FastComments widget, pa nasljeđuju sve globalne prilagodbe koje ste primijenili.

### Lenja inicijalizacija

Prozori chata se inicijalizuju pri hover-u za desktop korisnike ili odmah kada su kreirani. Ovo smanjuje početno opterećenje učitavanja tako što renderuje interfejs chata samo kada korisnici zaista interaguju sa markerom.

Lenja inicijalizacija se događa transparentno. Korisnici ne primjete kašnjenje, ali browser ne mora renderovati desetine skrivenih prozora chata ako imate mnogo markera na slici.

### Lokalizacija

Image Chat podržava iste opcije lokalizacije kao standardni FastComments widget. Postavite opciju `locale` da prikažete UI tekst na različitim jezicima:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'fr'  // Francuski
});
```

FastComments podržava desetine jezika. Podešavanje locale utiče na sav UI tekst uključujući upite, dugmad i tekst zamjenskih mjesta.

### Naslijeđene opcije prilagođavanja

Pošto Image Chat proširuje standardni widget za komentare, nasljeđuje sve opcije prilagođavanja iz osnovnog widgeta. Ovo uključuje prilagođene CSS klase, prilagođene prevode, prilagođavanje avatara, formatiranje datuma i mnogo više.

Pogledajte glavnu dokumentaciju za prilagođavanje FastComments-a za kompletan spisak dostupnih opcija prilagođavanja.

### Rad sa prilagođenim fontovima

Ako vaš sajt koristi prilagođene fontove, Image Chat UI će naslijediti te fontove iz CSS-a vaše stranice. Prozori chata se renderuju unutar DOM-a vaše stranice i poštuju postojeće tipografske postavke.

Za najbolje rezultate, osigurajte da su vaši prilagođeni fontovi učitani prije inicijalizacije Image Chat-a, ili prihvatite da može doći do kratkog prikaza nestyle-ovanog teksta dok se fontovi učitavaju.

### Vizuelni dizajn markera

Kvadratni markeri imaju suptilan vizuelni dizajn koji ih čini uočljivim bez preplavljivanja slike. Možete prilagoditi njihov izgled pomoću CSS-a ako želite drugačiju vizuelnu obradu.

Markeri uključuju hover state-ove koji pružaju povratnu informaciju kada korisnici pomjere miš preko njih. Na uređajima sa touch ekranom, dodir pruža trenutnu povratnu informaciju otvaranjem prozora chata.