[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

Sa FastComments, sav tekst u vidžetu za komentare je prilagodljiv.

Možete zameniti pojedinačni deo teksta, kao što je dugme za slanje, ili sav tekst u celom vidžetu za komentare.

Podrazumevano, tekst u vidžetu za komentare se prevodi prema lokalitetu korisnika. Međutim, možemo zameniti tekst ako smo sigurni da naša baza korisnika koristi isti lokal/jezik, na primer:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Prilagođeni tekst'; code-example-end]

Sve prilagodljive prevode možete pronaći <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">ovde</a> pod karticom „advanced options“ tab.

Međutim, postoji jednostavniji način, putem UI-ja za prilagođavanje vidžeta. Tamo možemo jednostavno pronaći tekst koji se prikazuje u vidžetu za komentarisanje na EN_US lokalitetu i navesti zamenu.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; alt='Panel za prilagođeni tekst sa stringom vidžeta izabranim iz padajućeg menija i poljem za zamenu teksta'; title='Prilagođeni tekst' app-screenshot-end]

Sva prepisivanja prevoda trenutno utiču na sve lokalitete.