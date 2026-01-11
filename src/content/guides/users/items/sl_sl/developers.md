Za razvijalce, ki jim morda ne želite dodeliti vloge `Administrators`, razmislite o ustvarjanju uporabnika `Administrator`
with the following permissions:

1. Analytics Admin
2. Customizations Admin
3. Data Management Admin
4. Comment Moderation Admin
5. API/SSO Admin

This set of permissions will give a developer everything they need to setup FastComments as well
as the visibility into the system required to ensure it is working.

The reasoning for these permissions is as follows:

1. **Analytics Admin**: To se lahko uporabi za odpravljanje težav pri uporabi FastComments.
2. **Customizations Admin**: To bo potrebno za uporabo prilagojenega oblikovanja komentarnega vtičnika.
3. **Data Management Admin**: To bo potrebno za upravljanje uvozov in izvozov ter nastavitev webhookov.
4. **Comment Moderation Admin**: To bo potrebno za ogled podatkov o komentarjih, vsaj med nastavitvijo.
5. **API/SSO Admin**: To jim bo omogočilo neposredno pridobivanje API keys s naše platforme. We consider
this more secure than an `Administrator` copying it for them and sending the API Secret via email which
   may not be very secure.