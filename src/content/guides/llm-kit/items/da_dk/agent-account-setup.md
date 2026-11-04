En kodningsagent såsom Claude Code, Cursor eller en MCP-baseret assistent kan sætte FastComments op for dig, uden at du skal udfylde tilmeldingsformularen. Dette er nyttigt, når du beder en agent om at "tilføje kommentarer til mit site", og du endnu ikke har en konto.

### Sådan fungerer det

1. Agenten opretter en ny konto og modtager en API-nøgle og et claim-link. API-nøglen virker med det samme, så agenten kan konfigurere kontoen og installere widget'en på dit site.
2. Agenten giver dig claim-linket. Åbn det i din browser, log ind eller opret et login, og bekræft claimet. Kontoen er derefter din: du administrerer den, dens fakturering og dens API-nøgler fra dashboardet. Siden viser den API-nøgle, som agenten har, så du kan tilbagekalde den, hvis du ikke længere ønsker, at agenten – eller den, der driver den – skal have adgang.
3. Hvis ingen åbner claim-linket inden for 72 timer, slettes kontoen og dens nøgle. Bed agenten om at oprette en ny.

Indtil den bliver claimet, har kontoen de samme begrænsninger som en normal gratis prøveperiode.

### Hvis du allerede har en konto

Hvert login ejer én konto. Hvis du åbner et claim-link, mens du er logget ind på en eksisterende konto, giver siden dig mulighed for at vælge:

- **Tilknyt min konto** gør den nye konto til en administreret tenant under den konto, du er logget ind på. Dette kræver en betalt plan med white labeling, og den nye tenants forbrug faktureres til din konto.
- **Log ud og claim med et andet login** logger dig ud og bringer dig tilbage til claim-siden, så du kan claimet den med et andet login.

### Til agentforfattere

Agentinstruktionerne på [fastcomments.com/agents.md](https://fastcomments.com/agents.md) beskriver kaldet til oprettelse af konto, felterne i svaret og hvordan claim-linket overdrages til den person, du arbejder for. Kaldet kræver ingen API-nøgle og er rate-begrænset pr. IP-adresse.