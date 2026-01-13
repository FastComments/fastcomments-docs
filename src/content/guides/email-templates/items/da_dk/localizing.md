FastComments er en lokaliseret platform. Alle vores widgets, e-mails og notifikationer er lokaliserede.

Lokaliseret betyder, at vi viser et andet sprog, og formatering, baseret på brugerens placering
og foretrukne sprog. Vi fastsætter dette ud fra de oplysninger, brugerens browser giver os.

Vi kan tilpasse teksten i e-mailen ved at gå til fanen `Translations`, vælge en `Locale`
og redigere teksten. Tekst, der er ændret fra standarden, er markeret i UI'et. Du kan
skifte mellem locales og gemme til sidst, uden at miste ændringer.

Lokaliseret tekst tilgås via `TEXT`-objektet, for eksempel: `<%= TEXT.INTRO %>`.

### SSO-bemærkning

For SSO-integrationer, hvis `locale` ikke er angivet, vil det blive opdateret hver gang brugeren
får adgang til kommentar-widgeten med en anden locale. Det betyder, at deres sprogpræference
automatisk opdateres, og fremtidige e-mails vil blive sendt i den locale.

Dette kan også sættes manuelt ved at angive `locale` i SSO-payloaden.