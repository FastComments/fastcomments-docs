[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

S FastCommentsom, sav tekst u widgetu za komentare je prilagodljiv.

Možete zamijeniti pojedini dio teksta, poput gumba za slanje, ili sav tekst u cijelom widgetu za komentare.

Po zadanom se tekst u widgetu za komentare prevodi prema korisničkoj lokalnoj postavci. Međutim, možemo prebrisati tekst, ako smo sigurni
da naša baza korisnika koristi istu lokalnu postavku/jezik, na primjer:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

Sve prilagodljive prijevode možete pronaći <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">ovdje</a> pod karticom "advanced options".

Međutim, postoji lakši način, putem korisničkog sučelja za prilagodbu widgeta. Tamo možemo jednostavno pronaći tekst koji se prikazuje u widgetu za komentare u EN_US lokalizaciji, i navesti
zamjenski tekst.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; title='Custom Text' app-screenshot-end]

Sve zamjene prijevoda trenutno utječu na sve lokalizacije.