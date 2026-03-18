Direct Messaging (DM) giver FastComments-brugere mulighed for private, én-til-én samtaler med hinanden, adskilt fra offentlige kommentarer og profilinteraktioner.

### Hvad er Direct Messaging?

Direct Messaging giver:
- **Private samtaler** mellem to brugere
- **Realtime-beskeder** med notifikationer
- **Samtalehistorik** der bevares på tværs af sessioner
- **Online-status** indikatorer, så du kan se, hvornår en person er tilgængelig
- **Sporing af ulæste beskeder** så du kan holde styr på nye beskeder

### Sådan starter du en Direct Message

Der er flere måder at starte en DM-samtale på:

**Fra en brugers profil:**
1. Gå til brugerens profilside
2. Klik på fanen "Direkte beskeder" eller knappen "Send besked"
3. Skriv din besked og send

**Fra en kommentar:**
Nogle implementeringer tillader at klikke på en brugers navn eller avatar for hurtigt at få adgang til deres profil og beskedmuligheder.

**Direkte link:**
Hvis du har et samtale-id eller et DM-videresendelseslink, kan du få adgang til samtaler direkte.

### Adgang til dine beskeder

For at se alle dine direkte samtaler:

1. Gå til din profilside
2. Klik på fanen "Direkte beskeder"
3. Se en liste over alle dine samtaler

Hver samtale viser:
- Den anden deltageres avatar og navn
- Deres online/offline-status (grøn indikator når online)
- En forhåndsvisning af den seneste besked
- Antal ulæste beskeder (hvis nogen)
- Tidsstempel for seneste aktivitet

### Samtalevisning

Når du åbner en samtale, vil du se:

- **Fuld beskedhistorik** - Alle beskeder mellem dig og den anden bruger
- **Realtime-opdateringer** - Nye beskeder vises øjeblikkeligt via WebSocket
- **Online-status** - Se om den anden person er online lige nu
- **Tidsstempler på beskeder** - Hvornår hver besked blev sendt
- **Beskedkomposition** - Tekstfelt til at skrive og sende nye beskeder

### Beskednotifikationer

Du vil modtage notifikationer for nye direkte beskeder:

- **I-app-notifikationer** - Badge-tæller på din profil
- **E-mail-notifikationer** - Afhængigt af dine notifikationsindstillinger
- **Realtime-advarsler** - Øjeblikkelige notifikationer når du er logget ind

Administrer notifikationspræferencer i dine [Kontoindstillinger](https://fastcomments.com/auth/my-account/edit-notifications).

### Privatliv og blokering

**Deaktivering af direkte beskeder:**
Hvis du ikke ønsker at modtage DMs, kan du deaktivere dem fuldstændigt:
1. Gå til Privatindstillinger på din profil
2. Aktivér "Deaktiver direkte beskeder"
3. DM-muligheden vil blive skjult fra din profil

For flere oplysninger, se [Privatindstillinger](/guide-user-profiles.html#privacy-settings).

**Blokering af brugere:**
Hvis nogen chikanerer dig via DM:
1. Gå til deres profil
2. Bloker brugeren
3. De vil ikke længere kunne sende dig beskeder

Blokerede brugere kan ikke:
- Sende dig direkte beskeder
- Se din aktivitet
- Kommentere på din profil
- Interagere med dig nogen steder på FastComments

### Håndtering af samtaler

**Skjule samtaler:**
Du kan skjule samtaler, du ikke længere vil se i din samtaleliste:
1. Åbn din fane Direkte beskeder
2. Find den samtale, du vil skjule
3. Vælg skjul/arkivér-udgaven

Skjulte samtaler vises ikke i din hovedliste, men kan tilgås, hvis den anden person sender en ny besked.

**Markér som læst:**
Beskeder markeres automatisk som læst, når du ser dem. Du kan også manuelt markere hele samtaler som læst:
1. Åbn din fane Direkte beskeder
2. Vælg en samtale
3. Vælg "Markér som læst"

Dette nulstiller tælleren over ulæste beskeder for den samtale.

### Bedste praksis

**Hvornår du skal bruge Direct Messages:**
- Stille opfølgende spørgsmål privat
- Takke nogen for nyttig rådgivning
- Diskutere off-topic emner uden at rode de offentlige kommentarer til
- Koordinere med andre medlemmer af fællesskabet
- Give privat feedback eller forslag

**DM-etikette:**
- Vær respektfuld og professionel
- Spam ikke brugere med uopfordrede beskeder
- Respekter, hvis nogen ikke svarer eller deaktiverer DMs
- Hold samtaler relevante og konstruktive
- Del ikke andres DMs uden tilladelse

**Sikkerhedstips:**
- Del ikke personlige oplysninger (telefon, adresse osv.), medmindre du stoler på personen
- Bloker og rapporter brugere, der chikanerer eller misbruger DMs
- Vær forsigtig med mistænkelige links eller forespørgsler
- Brug blokering, hvis nogen gør dig utilpas

### Begrænsninger og noter

**Hvem kan du sende besked til:**
- Enhver FastComments-bruger, der ikke har deaktiveret DMs
- Brugere, som ikke har blokeret dig
- Brugere på tværs af alle FastComments-fællesskaber

**Beskedindhold:**
- Tekstbeskeder understøttes
- Beskeder følger de samme indholdsregler som kommentarer
- Upassende indhold kan rapporteres

**Samtaleomfang:**
- DM-samtaler er private mellem to personer
- Ingen gruppesamtaler (i øjeblikket kun én-til-én)
- Samtalehistorik bevares på ubestemt tid

### Fejlfinding

**Kan du ikke sende en besked?**
Den anden bruger kan have:
- Deaktiveret direkte beskeder i deres privatindstillinger
- Blokeret dig
- Slettet deres konto

**Modtager du ikke notifikationer?**
Tjek dine notifikationsindstillinger på [Kontoindstillinger](https://fastcomments.com/auth/my-account/edit-notifications) for at sikre, at DM-notifikationer er aktiveret.

**Beskeder sender ikke?**
- Tjek din internetforbindelse
- Opdater siden og prøv igen
- Sørg for, at du ikke er blokeret
- Kontakt support, hvis problemet fortsætter