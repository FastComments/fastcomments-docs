FastComments giver dig mulighed for at kræve, at første‑gangs kommentatorer accepterer dine Servicevilkår, før de indsender en kommentar.

Når aktiveret:
- **Anonyme brugere** vil se en TOS‑afkrydsningsboks hver gang de kommenterer
- **Godkendte brugere** vil kun se afkrydsningsboksen på deres første kommentar, eller når du opdaterer dine TOS

### Konfiguration

Naviger til widget‑tilpasningssiden og aktivér afkrydsningsfeltet "Kræv accept af Servicevilkår". Når den er aktiveret, vil du se følgende indstillinger:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; alt='Servicevilkår-panel, der viser TOS-teksttilstandsvælgeren og feltet for seneste opdateringsdato'; title='Indstillinger for servicevilkår' app-screenshot-end]

- **TOS Teksttilstand**: Som standard viser afkrydsningsboksen "Jeg accepterer Servicevilkårene og Privatlivspolitikken" med links til begge dokumenter. Vælg "Tilpas tekst pr. locale" for at angive din egen tekst for hvert sprog.
- **TOS Seneste opdateringsdato**: Når du opdaterer dine Servicevilkår, angiv denne dato. Brugere, der accepterede før denne dato, vil blive bedt om at acceptere igen.

### Sådan fungerer det

- TOS‑accept‑tidsstemplet gemmes per‑bruger og per‑kommentar
- Når en bruger accepterer TOS, registreres datoen på deres brugerprofil (per‑tenant)
- Hvis du angiver en "Seneste opdateret"-dato, der er efter brugerens acceptdato, skal de acceptere igen
- For anonyme brugere, som ikke kan spores, vises afkrydsningsboksen ved hver kommentarindsendelse