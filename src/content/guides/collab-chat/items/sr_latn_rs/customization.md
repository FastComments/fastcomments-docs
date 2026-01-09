### Dark Mode Support

### Dynamic Dark Mode

If your site's dark mode is controlled by adding a `.dark` class to the body element, the Collab Chat korisnički interfejs će to automatski poštovati bez potrebe za reinicijalizacijom. Stilovi widgeta su dizajnirani da reaguju na prisustvo ove klase.

[inline-code-attrs-start title = 'Primer CSS-a za tamni režim'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/* Vaš CSS za tamni režim */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
[inline-code-end]

### Custom Styling with CSS

Možete prilagoditi izgled isticanja teksta, prozora četa i drugih elemenata koristeći CSS. Widget dodaje specifične klase koje možete ciljati u vašem stylesheet-u.

Isticanje teksta koristi sistem stilova mehurića komentara FastComments, tako da bilo kakve prilagodbe koje ste primenili na standardni widget za komentare takođe će uticati na Collab Chat.

### Top Bar Customization

Gornja traka prikazuje broj korisnika online i broj diskusija. Možete prilagoditi njenu poziciju pružanjem prilagođenog elementa kao `topBarTarget`:

[inline-code-attrs-start title = 'Prilagođena lokacija gornje trake'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('my-custom-header')
});
[inline-code-end]

Or disable it entirely by setting it to `null`:

[inline-code-attrs-start title = 'Onemogući gornju traku'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});
[inline-code-end]

### Mobile Behavior

On screens under 768px wide, Collab Chat automatically switches to a mobile-optimized layout. Chat windows appear fullscreen instead of floating next to the text, and the selection delay is removed for more immediate interaction.

Ovo ponašanje je ugrađeno i ne zahteva nikakvu konfiguraciju. Widget automatski detektuje veličinu ekrana i prilagođava se u skladu sa tim.

### Chat Window Appearance

Chat windows are 410px wide on desktop with a 16px arrow pointing to the highlighted text. The windows position themselves automatically based on available viewport space, using positioning classes like `to-right`, `to-left`, `to-top`, and `to-bottom`.

Možete dodati prilagođeni CSS da podesite boje, fontove, razmake ili druge vizuelne osobine ovih prozora. Prozori četa koriste istu strukturu komponenti kao standardni FastComments widget, tako da nasleđuju sve globalne prilagodbe koje ste primenili.

### Localization

Collab Chat podržava sve iste opcije lokalizacije kao standardni FastComments widget. Podesite opciju `locale` da prikažete tekst korisničkog interfejsa na različitim jezicima:

[inline-code-attrs-start title = 'Podesite lokal'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Španski
});
[inline-code-end]

FastComments podržava desetine jezika. Podešavanje locale utiče na sav tekst korisničkog interfejsa uključujući upite, dugmad i tekst u poljima za unos.

### Inherited Customization Options

Pošto Collab Chat proširuje standardni widget za komentare, nasleđuje sve opcije prilagođavanja iz osnovnog widgeta. Ovo uključuje prilagođene CSS klase, prilagođene prevode, prilagođavanje avatara, formatiranje datuma i mnogo više.

Pogledajte glavnu FastComments dokumentaciju o prilagođavanju za kompletan spisak dostupnih opcija prilagođavanja.

### Working with Custom Fonts

Ako vaš sajt koristi prilagođene fontove, Collab Chat korisnički interfejs će naslediti te fontove iz CSS-a vaše stranice. Možda ćete morati da kreirate pravilo za prilagođavanje widgeta i `@import` bilo koje fontove u prilagođenom CSS-u unutar tog pravila ako želite da plutajući prozor četa koristi iste fontove.