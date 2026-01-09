### Podrška za tamni režim

Image Chat uključuje ugrađenu podršku za tamni režim. Kada postavite `hasDarkBackground: true` u svojoj konfiguraciji, prozori chata i UI elementi se automatski prilagođavaju da dobro funkcionišu na tamnim pozadinama.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

Stilovi za tamni režim primenjuju se na prozore chata, marker kvadrate i sve interaktivne elemente. Ako vaš sajt ima prekidač za tamni režim, možete ponovo inicijalizovati widget kada se režim promeni, ili koristiti pristup sa klasom na body elementu opisan ispod.

### Dinamički tamni režim

Ako je tamni režim na vašem sajtu kontrolisan dodavanjem klase `.dark` na body element, Image Chat UI će to automatski poštovati bez potrebe za ponovnom inicijalizacijom. Stilovi widgeta su dizajnirani da reaguju na prisustvo ove klase.

```css
/* Vaš CSS za tamni režim */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
```

### Prilagođavanje stilova pomoću CSS-a

Možete prilagoditi izgled markera, prozora chata i drugih elemenata koristeći CSS. Widget dodaje specifične klase koje možete ciljati u vašem stylesheet-u.

Chat kvadratići i prozori koriste FastComments sistem stilizovanja komentarskih balona, tako da će sve prilagodbe koje ste primenili na standardni komentar widget takođe uticati na Image Chat.

### Veličina chat kvadrata

Opcija `chatSquarePercentage` kontroliše veličinu klikabilnih markera. Podrazumevana vrednost je 5% širine slike:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 7  // Veći, vidljiviji kvadratići
});
```

Manje vrednosti stvaraju nežnije markere koji se bolje uklapaju u sliku. Veće vrednosti čine markere istaknutijim i lakšim za klik, posebno na mobilnim uređajima ili radi pristupačnosti.

### Ponašanje na mobilnim uređajima

Na ekranima širine manje od 768px, Image Chat se automatski prebacuje na layout optimizovan za mobilne uređaje. Prozori chata se pojavljuju preko celog ekrana umesto da lebde pored markera, što pruža bolju upotrebljivost na malim ekranima.

Markeri ostaju vidljivi na svojim responzivnim pozicijama na slici. Korisnici mogu dodirnuti bilo koji marker da otvore fullscreen chat interfejs. Ovo ponašanje je ugrađeno i ne zahteva nikakvu konfiguraciju.

### Izgled prozora chata

Prozori chata su široki 300px na desktopu sa strelicom od 16px koja pokazuje na marker. Prozori se automatski pozicioniraju u zavisnosti od dostupnog prostora u viewport-u, koristeći klase pozicioniranja kao što su `to-right`, `to-left`, `to-top`, i `to-bottom`.

Možete dodati prilagođeni CSS da podesite boje, fontove, razmake ili druge vizuelne osobine ovih prozora. Prozori chata koriste istu strukturu komponenti kao standardni FastComments widget, tako da nasledjuju sve globalne prilagođavanja koja ste primenili.

### Lenja inicijalizacija

Prozori chata se inicijalizuju na hover za desktop korisnike ili odmah kada su kreirani. Ovo smanjuje početno opterećenje učitavanja tako što renderuje chat interfejs samo kada korisnici zaista interaguju sa markerom.

Lenja inicijalizacija se dešava transparentno. Korisnici ne primete kašnjenje, ali pregledač ne mora da renderuje desetine skrivenih prozora chata ako imate mnogo markera na slici.

### Lokalizacija

Image Chat podržava sve iste opcije lokalizacije kao standardni FastComments widget. Podesite opciju `locale` da prikažete UI tekst na različitim jezicima:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'fr'  // Francuski
});
```

FastComments podržava desetine jezika. Podešavanje locale utiče na sav UI tekst uključujući poruke, dugmad i tekst u placeholderima.

### Nasleđene opcije prilagođavanja

Pošto Image Chat proširuje standardni komentar widget, nasleđuje sve opcije prilagođavanja iz osnovnog widgeta. To uključuje prilagođene CSS klase, prilagođena prevoda, prilagođavanje avatara, formatiranje datuma i još mnogo toga.

Pogledajte glavnu FastComments dokumentaciju o prilagođavanju za kompletan spisak dostupnih opcija prilagođavanja.

### Rad sa prilagođenim fontovima

Ako vaš sajt koristi prilagođene fontove, Image Chat UI će naslediti te fontove iz CSS-a vaše stranice. Prozori chata se renderuju unutar DOM-a vaše stranice i poštuju postojeća tipografska podešavanja.

Za najbolje rezultate, osigurajte da su vaši prilagođeni fontovi učitani pre inicijalizacije Image Chat-a, ili prihvatite da može doći do kratkog treptaja nestiilisanog teksta dok se fontovi učitavaju.

### Vizuelni dizajn markera

Kvadratni markeri imaju suptilan vizuelni dizajn koji ih čini uočljivim bez preopterećenja slike. Možete prilagoditi njihov izgled pomoću CSS-a ako želite drugačiji vizuelni tretman.

Markeri uključuju hover stanja koja daju povratnu informaciju kada korisnici pomere miš preko njih. Na uređajima osetljivim na dodir, tap interakcija daje momentalnu povratnu informaciju otvaranjem prozora chata.

---