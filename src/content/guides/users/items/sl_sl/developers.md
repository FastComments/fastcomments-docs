Za razvijalce, ki jih morda ne želite imeti kot `Administratorje`, razmislite o ustvarjanju uporabnika `Administrator`
z naslednjimi dovoljenji:

1. Analytics Admin
2. Customizations Admin
3. Data Management Admin
4. Comment Moderation Admin
5. API/SSO Admin

Ta nabor dovoljenj bo razvijalcu dal vse, kar potrebuje za nastavitev FastComments, kot tudi
vidljivost v sistem, potrebno za zagotovitev, da deluje.

Razlog za ta dovoljenja je naslednji:

1. **Analytics Admin**: To lahko uporabite za odpravljanje napak pri uporabi FastComments.
2. **Customizations Admin**: To bo potrebno za uporabo prilagojenega stila pripomočka za komentarje.
3. **Data Management Admin**: To bo potrebno za upravljanje uvozov in izvozov ter nastavitev webhookov.
4. **Comment Moderation Admin**: To bo potrebno za ogled podatkov o komentarjih, vsaj med nastavitvijo.
5. **API/SSO Admin**: To jim bo omogočilo, da API ključe pridobijo neposredno z naše platforme. Menimo,
da je to varneje kot to, da `Administrator` ključ skopira zanje in pošlje API skrivnost po e-pošti,
kar morda ni posebej varno.