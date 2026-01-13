Za developere za koje možda ne želite da budu `Administrators`, razmislite o stvaranju `Administrator` korisnika s sljedećim dozvolama:

1. Analytics Admin
2. Customizations Admin
3. Data Management Admin
4. Comment Moderation Admin
5. API/SSO Admin

Ovaj skup dozvola dat će developeru sve što mu treba za postavljanje FastCommentsa, kao i uvid u sustav potreban za provjeru ispravnog rada.

Razlozi za ove dozvole su sljedeći:

1. **Analytics Admin**: Ovo se može koristiti za otklanjanje problema pri korištenju FastCommentsa.
2. **Customizations Admin**: Ovo će biti potrebno za primjenu prilagođenog stila na widget za komentare.
3. **Data Management Admin**: Ovo će biti potrebno za upravljanje uvozima i izvozima, te postavljanje webhooks.
4. **Comment Moderation Admin**: Ovo će biti potrebno kako bi se vidjeli podaci o komentarima, barem tijekom postavljanja.
5. **API/SSO Admin**: Ovo će im omogućiti dohvat API ključeva izravno s naše platforme. Smatramo da je to sigurnije nego da `Administrator` kopira iste za njih i šalje API Secret putem e-pošte, što možda nije vrlo sigurno.