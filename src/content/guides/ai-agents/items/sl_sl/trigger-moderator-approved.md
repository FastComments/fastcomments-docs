---
Sproži se, ko moderator odobri komentar.

### Kontekst, ki ga prejme agent

- Pravkar odobren komentar.
- **ID sprožilnega uporabnika** - moderator, ki je odobril.
- Neobvezna zgodovina niti / uporabnika / kontekst strani, kot je konfigurirano.

### Kdo to sproži

Dejanje človeškega moderatorja.

### Pomembno

- Komentar "approved" je v terminologiji FastComments **viden** komentar. Glejte [How Approvals Work](/guide-moderation.html#moderation-approvals) v priročniku za moderiranje za razliko med approved/unapproved in reviewed/unreviewed.
- Sprožilec se aktivira ob odobritvenih **prehodih**: komentar, ki preide iz unapproved v approved, ga sproži; komentar, ki je bil že approved in se ponovno shrani, ga ne sproži.
- Za najemnike, kjer so komentarji privzeto samodejno odobreni, se ta sprožilec sproži le, ko moderator eksplicitno ponovno odobri prej skrit komentar.

### Pogoste uporabe

- **Dobrodošlica / angažiranost** - agent se lahko odzove na prve komentatorje v trenutku, ko jih moderator odobri, namesto ob objavi.
- **Sodelovanje med agenti** - če je drug agent označil komentar za pregled, je odobritev znak, da je človeški pregled končan.
- **Revizijska sled** preko [Webhooks](#webhooks-overview).

---