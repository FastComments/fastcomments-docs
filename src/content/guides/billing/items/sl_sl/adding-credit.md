Dopolnitve se bremenijo na plačilni način, shranjen na vašem računu. Če še niste shranili kartice, stran Credits prikaže `Add a payment method first` z gumbom `Go to billing`. Shranite kartico na strani [Billing Info](https://fastcomments.com/auth/my-account/billing-info) in se nato vrnite.

Za dodajanje kredita:

1. Odprite [My Account -> Billing Info](https://fastcomments.com/auth/my-account/billing-info) in kliknite `Manage Credits`.
2. Pod `Add credit` izberite eno izmed prednastavljenih zneskov ali izberite `Other amount` in vnesite svojega.
3. Kliknite gumb `Add`. Ta prikaže izbrani znesek, na primer `Add $25.00`.
4. Če vaša banka zahteva potrditev plačila, dokončajte preverjanje v oknu, ki se odpre.

Ko plačilo uspe, stran prikaže `Credit added` z vašim novim stanjem, dopolnitev pa se prikaže v `Past top-ups` kot `Credited`. Kliknite `Add more` za novo dopolnitev.

### Zneski in omejitve

Prednastavljeni zneski so `10`, `25`, `50` in `100` v vaši obračunski valuti. Poljubni znesek je lahko od `5` do `10,000` na dopolnitev, točne omejitve za vašo valuto so prikazane pod obrazcem. Pri valutah brez decimalnih mest, kot je japonski jen, so prednastavljeni zneski in omejitve stokrat večji, da predstavljajo približno enako vrednost.

Dopolnitve so vedno v obračunski valuti vašega računa, isti valuti kot vaši računi. Stran vam pove, katera valuta bo bremenjena.

### Potrditev banke (3D Secure)

Nekatere banke zahtevajo potrditev spletnega plačila, običajno prek bančne aplikacije ali enkratne kode. Če to velja za vašo banko, se po kliku na `Add` odpre okno za potrditev. Vaša kartica je bremenjena in stanje posodobljeno šele, ko banka odobri.

Če zaprete stran pred zaključkom potrditve, se dopolnitev zabeleži kot `Pending`. Naslednjič, ko odprete stran Credits, preverimo status plačila pri Stripe in ga označimo kot `Credited` ali `Failed`. Če banka plačila ne odobri, stran prikaže `Your bank did not approve the payment. Try again, or use a different card.` in nič ni bremenjeno.

### Zavrnjene kartice

Če je vaša kartica zavrnjena, stran prikaže `Your card was declined. Check your card on the billing page, or try another card.` Nič ni bremenjeno, vaše stanje se ne spremeni, poskus pa je naveden v `Past top-ups` kot `Failed`. Posodobite kartico na strani [Billing Info](https://fastcomments.com/auth/my-account/billing-info) in poskusite znova.

Za zaščito vaše kartice lahko račun na uro opravi omejeno število poskusov dopolnitve. Če vidite `Too many top-up attempts. Wait a while and try again.`, počakajte in poskusite pozneje.

Plačila obdeluje Stripe. FastComments nikoli ne shranjuje podatkov o vaši kartici.