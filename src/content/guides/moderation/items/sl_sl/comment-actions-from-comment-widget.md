Podnabor ukrepov moderiranja je mogoče izvesti neposredno iz niza komentarjev, brez potrebe po obisku strani za moderacijo komentarjev.

Ko ste prijavljeni, kliknite gumb za urejanje v zgornjem desnem kotu komentarja. Kot moderator bi morali imeti naslednje možnosti:

- **Pripni** ta komentar
- **Izbriši** ta komentar
- **Izbriši** ta komentar + **Blokiraj uporabnika** (Trajno ali Shadow, več podrobnosti kasneje)
- **Uredi** ta komentar
- **Zakleni** ali **Odkleni** ta komentar (več podrobnosti spodaj)
- Označi ta komentar kot **Odobreno** (prikaži) ali **Neodobreno** (skrij)
- Označi ta komentar kot **Spam** ali **Ne-Spam**

### Zaklepanje komentarja

Zaklepanje posameznega komentarja prepreči kakršnekoli nove odgovore nanj in tudi prepreči, da bi bil komentar urejen ali izbrisan, dokler ni odklenjen. To velja za vse, vključno z admini in moderatorji. Če morate urediti ali odstraniti zaklenjen komentar, ga najprej odklenite, izvedite spremembo in ga po želji ponovno zaklenite.

V zgornjem desnem kotu zaklenjenega komentarja se prikaže ikona ključavnice, da lahko bralci na prvi pogled vidijo, da je nit zaprta. Vnosov menija Uredi in Izbriši ni mogoče videti za zaklenjene komentarje tako v pripomočku za komentarje kot v javnem API-ju (`PATCH` in `DELETE` vrneta `code: 'locked'`, če se kličejo na zaklenjen komentar).

Dve namenski izjemi zaobideta zaklep, ker bi sicer pustili osirotene podatke za sabo: ko uporabnik izbriše svoj račun v celoti (njegovi komentarji se počistijo ne glede na stanje zaklepa) in ko moderator blokira uporabnika z možnostjo "izbriši vse komentarje tega uporabnika" (ta čiščenje odstrani tudi zaklepe).

### Zapiranje niti komentarjev

Moderatorji in skrbniki lahko zaklenejo oziroma zaprejo niti komentarjev, tako da v zgornjem delu področja komentarjev, če so prijavljeni, izberejo `Close Thread` v meniju s tremi pikami. Kasneje lahko kadar koli izberejo `Re-Open Thread`, da ponovno odprejo komentiranje.

Zapiranje niti komentarjev prepreči nove komentarje, vendar še vedno dovoljuje glasovanje in uporabnikom omogoča brisanje njihovih komentarjev, če to želijo.

Zapiranje in ponovno odpiranje niti komentarjev takoj vpliva na vse uporabnike, ki si ogledujejo to nit.

Nit lahko tudi označite kot samo za branje, kar odstrani možnosti glasovanja in brisanja, tako da ustvarite pravilo prilagoditve posebej za to stran.

### Posodobitve v živo

Vse te akcije bodo takoj posodobile niti komentarjev pri drugih uporabnikih, ne da bi morali osvežiti stran. Vendar pa moderatorjeve akcije, kot sta skrivanje komentarja ali označevanje kot spam, komentarja ne odstranijo z zaslona moderatorja, tako da ga lahko po potrebi hitro razveljavijo. Da se nakaže, da je komentar skrit, bo v primerjavi z drugimi komentarji označen (barva označbe je odvisna od razloga odstranitve).

Na primer, ob predpostavki uporabnikov `A (commenter)`, `B (Moderator 1)` in `C (Moderator 2)`.

...in naslednji scenarij:

1. `User B (Moderator 1)` skrije komentar.
2. Za `User A (commenter)` je ta komentar takoj skrit.
3. Za `User C (Moderator 2)` je ta komentar takoj skrit.
4. Za uporabnika, ki je izvedel spremembo, `User B (Moderator 1)`, komentar ostane na njihovem zaslonu, vendar je označen kot odstranjen. Imajo možnost razveljaviti svojo akcijo, v tem primeru bodo drugi uporabniki takoj ponovno videli posodobitev v živo.