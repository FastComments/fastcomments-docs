[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

Sa FastComments, sav tekst u widgetu za komentare je prilagodljiv.

Možete prepisati pojedinačni deo teksta, kao što je dugme za slanje, ili sav tekst u celom widgetu za komentare.

Podrazumevano, tekst u widgetu za komentare se prevodi na osnovu lokalizacije korisnika. Međutim, možemo prepisati tekst, ako smo sigurni
da naša baza korisnika koristi isti lokal/jezik, na primer:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Prilagođeni tekst'; code-example-end]

Sva prilagodljiva prevođenja možete pronaći <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">ovde</a> pod karticom "advanced options".

Međutim, postoji jednostavniji način, putem UI-ja za prilagođavanje widgeta. Tamo možemo jednostavno pronaći tekst koji se prikazuje u widgetu za komentarisanje u EN_US lokalizaciji i navesti zamenu.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; alt='Panel prilagođenog teksta sa stringom widgeta izabranim iz padajućeg menija i poljem za zamenu teksta'; title='Prilagođeni tekst' app-screenshot-end]

Trenutno, sva prepisivanja prevoda utiču na sve lokalizacije.