FastComments giver dig mulighed for at kræve, at førstegangskommenterende accepterer dine Servicevilkår, før de indsender en kommentar.

Når det er aktiveret:
- **Anonyme brugere** vil se et afkrydsningsfelt for Servicevilkår hver gang de kommenterer
- **Autentificerede brugere** vil se afkrydsningsfeltet kun ved deres første kommentar, eller når du opdaterer dine Servicevilkår

### Aktivering af Servicevilkår

Gå til siden til tilpasning af widget og aktiver afkrydsningsfeltet "Require Terms of Service acceptance":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-enabled'; title='Enable Terms of Service Checkbox' app-screenshot-end]

### Tilpasning af Servicevilkårsteksten

Som standard viser afkrydsningsfeltet "I agree to the Terms of Service and Privacy Policy" med links til begge dokumenter. Du kan tilpasse denne tekst pr. lokalitet, hvis nødvendigt:

1. Vælg "Customize text per locale"
2. Vælg lokaliteten fra dropdown-menuen og indtast din tilpassede tekst

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-text-mode'; title='Customize TOS Text' app-screenshot-end]

### Opdatering af dine Servicevilkår

Når du opdaterer dine Servicevilkår, angiv "Last Updated"-datoen. Brugere, som accepterede Servicevilkårene før denne dato, vil blive bedt om at acceptere igen:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-last-updated'; title='TOS Last Updated Date' app-screenshot-end]

### Hvordan det fungerer

- Tidsstemplet for accept af Servicevilkår gemmes pr. bruger og pr. kommentar
- Når en bruger accepterer Servicevilkårene, registreres datoen på deres brugerprofil (per-tenant)
- Hvis du angiver en "Last Updated"-dato, der ligger efter brugerens accepteringsdato, skal de acceptere igen
- For anonyme brugere, som ikke kan spores, vises afkrydsningsfeltet ved hver indsendelse af en kommentar