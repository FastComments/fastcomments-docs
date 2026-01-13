[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

U FastCommentsu, sav tekst u widgetu za komentare je prilagodljiv.

Možete zameniti jednu stavku teksta, kao što je dugme za slanje, ili ceo tekst u celom widgetu za komentare.

Podrazumevano, tekst u widgetu za komentare se prevodi u skladu sa lokalom korisnika. Međutim, možemo prepisati tekst, ako smo sigurni
da naša korisnička baza koristi isti lokal/jezik, na primer:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

Sve prilagodljive prevode možete pronaći <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">ovde</a> pod karticom "napredne opcije".

Međutim, postoji lakši način, preko UI-a za prilagođavanje widgeta. Tamo možemo jednostavno pronaći tekst koji se prikazuje u komentarskom widgetu u EN_US lokalu, i odrediti
zamenu.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; title='Custom Text' app-screenshot-end]

Sve izmene prevoda trenutno utiču na sve lokale.