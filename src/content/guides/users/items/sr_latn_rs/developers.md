Za developere koje možda ne želite da budu `Administrators`, razmislite o kreiranju `Administrator` korisnika
sa sledećim dozvolama:

1. Analytics Admin
2. Customizations Admin
3. Data Management Admin
4. Comment Moderation Admin
5. API/SSO Admin

Ovaj skup dozvola će developeru dati sve što mu je potrebno za podešavanje FastComments-a kao i
uvid u sistem potreban da se osigura da radi ispravno.

Razlozi za ove dozvole su sledeći:

1. **Analytics Admin**: Ovo se može koristiti za otklanjanje grešaka u korišćenju FastComments.
2. **Customizations Admin**: Ovo će biti potrebno da bi se primenio prilagođeni stil na comment widget.
3. **Data Management Admin**: Ovo će biti potrebno za upravljanje uvozima i izvozima, i podešavanje webhooks.
4. **Comment Moderation Admin**: Ovo će biti potrebno da bi se videli podaci o komentarima, bar tokom podešavanja.
5. **API/SSO Admin**: Ovo će im omogućiti da direktno preuzmu API keys sa naše platforme. Smatramo
da je ovo bezbednije nego da `Administrator` to kopira za njih i šalje API Secret putem email-a što
   možda nije baš bezbedno.