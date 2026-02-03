FastComments giver dig mulighed for at kræve, at nye kommentatorer accepterer dine vilkår for tjenesten, før de indsender en kommentar.

Når aktiveret:
- **Anonyme brugere** vil se et afkrydsningsfelt for tjenestevilkår hver gang de kommenterer
- **Autentificerede brugere** vil kun se afkrydsningsfeltet ved deres første kommentar, eller når du opdaterer dine tjenestevilkår

### Konfiguration

Gå til siden for widget-tilpasning, og aktivér afkrydsningsfeltet "Require Terms of Service acceptance". Når det er aktiveret, vil du se følgende indstillinger:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; title='Terms of Service Options' app-screenshot-end]

- **TOS Text Mode**: Som standard viser afkrydsningsfeltet "I agree to the Terms of Service and Privacy Policy" med links til begge dokumenter. Vælg "Customize text per locale" for at angive din egen tekst for hvert sprog.
- **TOS Last Updated Date**: Når du opdaterer dine tjenestevilkår, angiv denne dato. Brugere, der accepterede før denne dato, vil blive bedt om at acceptere igen.

### Sådan fungerer det

- Tidsstempel for TOS-accept gemmes pr. bruger og pr. kommentar
- Når en bruger accepterer tjenestevilkårene, registreres datoen på deres brugerprofil (per-tenant)
- Hvis du angiver en 'Last Updated'-dato, der er efter brugerens accepteringsdato, skal de acceptere igen
- For anonyme brugere, som ikke kan spores, vises afkrydsningsfeltet ved hver kommentarindsendelse

---