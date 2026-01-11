---
Direkte beskeder (DM) giver FastComments-brugere mulighed for at have private, én-til-én-samtaler med hinanden, adskilt fra offentlige kommentarer og profilinteraktioner.

### Hvad er direkte beskeder?

Direkte beskeder tilbyder:
- **Private samtaler** mellem to brugere
- **Realtidsbeskeder** med notifikationer
- **Samtalehistorik** som bevares på tværs af sessioner
- **Online-statusindikatorer** til at se, hvornår en person er tilgængelig
- **Sporing af ulæste beskeder** for at holde styr på nye beskeder

### Sådan starter du en direkte besked

Der er flere måder at starte en DM-samtale på:

**Fra en brugers profil:**
1. Besøg brugerens profilside
2. Klik på fanen "Direkte beskeder" eller knappen "Send besked"
3. Skriv din besked og send

**Fra en kommentar:**
Nogle implementeringer tillader, at man klikker på en brugers navn eller avatar for hurtigt at få adgang til deres profil og beskedmuligheder.

**Direkte link:**
Hvis du har en samtale-id eller et DM-viderestillingslink, kan du få direkte adgang til samtaler.

### Sådan får du adgang til dine beskeder

For at se alle dine direkte beskedsamtaler:

1. Besøg din profilside
2. Klik på fanen "Direkte beskeder"
3. Se en liste over alle dine samtaler

Hver samtale viser:
- Den anden deltagers avatar og navn
- Deres online/offline-status (grøn indikator når online)
- Et uddrag af den seneste besked
- Antal ulæste beskeder (hvis nogen)
- Tidsstempel for seneste aktivitet

### Samtalevisning

Når du åbner en samtale, vil du se:

- **Fuld beskedhistorik** - Alle beskeder mellem dig og den anden bruger
- **Realtidsopdateringer** - Nye beskeder dukker op øjeblikkeligt via WebSocket
- **Online-status** - Se om den anden person er online lige nu
- **Tidsstempler for beskeder** - Hvornår hver besked blev sendt
- **Beskedkomposition** - Tekstfelt til at skrive og sende nye beskeder

### Beskednotifikationer

Du vil modtage notifikationer for nye direkte beskeder:

- **In-app-notifikationer** - Badge-tæller på din profil
- **E-mail-notifikationer** - Afhængigt af dine notifikationsindstillinger
- **Realtidsalarmer** - Øjeblikkelige notifikationer når du er logget ind

Administrer dine notifikationspræferencer i dine [Kontoinstillinger](https://fastcomments.com/auth/my-account/notification-settings).

### Privatliv og blokering

**Deaktivering af direkte beskeder:**
Hvis du ikke ønsker at modtage DMs, kan du deaktivere dem helt:
1. Gå til privatlivsindstillinger på din profil
2. Aktiver "Deaktiver direkte beskeder"
3. DM-muligheden vil blive skjult fra din profil

For flere oplysninger, se [Privatlivsindstillinger](/guides/user-profiles/privacy-settings).

**Blokering af brugere:**
Hvis nogen chikanerer dig via DM:
1. Besøg deres profil
2. Bloker brugeren
3. De vil ikke længere kunne sende dig beskeder

Blokerede brugere kan ikke:
- Sende dig direkte beskeder
- Se din aktivitet
- Kommentere på din profil
- Interagere med dig nogen steder på FastComments

### Håndtering af samtaler

**Skjule samtaler:**
Du kan skjule samtaler, du ikke længere ønsker at se i din samtaleliste:
1. Åbn din fane for direkte beskeder
2. Find samtalen, du vil skjule
3. Vælg skjul/arkiver-indstillingen

Skjulte samtaler vises ikke i din hovedliste, men kan tilgås, hvis den anden person sender en ny besked.

**Markere som læst:**
Beskeder markeres automatisk som læst, når du ser dem. Du kan også manuelt markere hele samtaler som læst:
1. Åbn din fane for direkte beskeder
2. Vælg en samtale
3. Vælg "Marker som læst"

Dette rydder tælleren for ulæste beskeder for den samtale.

### Bedste praksis

**Hvornår du skal bruge direkte beskeder:**
- Stille opfølgende spørgsmål privat
- Takke nogen for nyttig hjælp
- Diskutere emner uden for emnet uden at fylde de offentlige kommentarer
- Koordinere med andre medlemmer af fællesskabet
- Give privat feedback eller forslag

**DM-etikette:**
- Vær respektfuld og professionel
- Send ikke spam eller uopfordrede beskeder til brugere
- Respekter, hvis nogen ikke svarer eller deaktiverer DMs
- Hold samtaler relevante og konstruktive
- Del ikke andres DMs uden tilladelse

**Sikkerhedstips:**
- Del ikke personlige oplysninger (telefon, adresse osv.), medmindre du stoler på personen
- Bloker og indberet brugere, der chikanerer eller misbruger DMs
- Vær forsigtig med mistænkelige links eller anmodninger
- Brug blokeringsfunktionen, hvis nogen gør dig utryg

### Begrænsninger og bemærkninger

**Hvem kan du sende besked til:**
- Enhver FastComments-bruger, som ikke har deaktiveret DMs
- Brugere, som ikke har blokeret dig
- Brugere på tværs af alle FastComments-fællesskaber

**Beskedindhold:**
- Tekstbeskeder understøttes
- Beskeder følger de samme indholdspolitikker som kommentarer
- Upassende indhold kan indberettes

**Samtaleomfang:**
- DM-samtaler er private mellem to personer
- Ingen gruppebeskeder (i øjeblikket kun én-til-én)
- Samtalehistorik bevares på ubestemt tid

### Fejlfinding

**Kan ikke sende en besked?**
Den anden bruger kan have:
- Deaktiveret direkte beskeder i deres privatlivsindstillinger
- Blokeret dig
- Slettet deres konto

**Modtager du ikke notifikationer?**
Tjek dine notifikationsindstillinger på [Kontoinstillinger](https://fastcomments.com/auth/my-account/notification-settings) for at sikre, at DM-notifikationer er aktiveret.

**Beskeder bliver ikke sendt?**
- Tjek din internetforbindelse
- Opdater siden og prøv igen
- Sørg for, at du ikke er blevet blokeret
- Kontakt support, hvis problemet vedbliver

---