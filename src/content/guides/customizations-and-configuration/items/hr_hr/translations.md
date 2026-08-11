[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

S FastComments, sav tekst u widgetu za komentare je prilagodljiv.

Možete zamijeniti pojedinačni dio teksta, poput gumba za slanje, ili sav tekst u cijelom widgetu za komentare.

Prema zadanim postavkama, tekst u widgetu za komentare prevodi se prema lokalizaciji korisnika. Međutim, možemo zamijeniti tekst ako smo sigurni
da naša baza korisnika koristi istu lokalizaciju/jezik, na primjer:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Prilagođeni tekst'; code-example-end]

All customizable translations can be found <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">ovdje</a> under the "advanced options" tab.

Međutim, postoji jednostavniji način, putem UI-ja za prilagodbu widgeta. Tamo možemo jednostavno pronaći tekst koji se prikazuje u widgetu za komentiranje u EN_US lokalizaciji i navesti
zamjenu.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; alt='Panel prilagođenog teksta s odabranom stringom widgeta iz padajućeg izbornika i poljem za zamjenski tekst'; title='Prilagođeni tekst' app-screenshot-end]

Sva prepisivanja prijevoda trenutno utječu na sve lokalizacije.