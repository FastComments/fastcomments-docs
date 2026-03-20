Direct Messaging (DM) tillader FastComments-brugere at have private, én-til-én samtaler med hinanden, adskilt fra offentlige kommentarer og profilinteraktioner.

### Hvad er Direct Messaging?

Direct Messaging tilbyder:
- **Private samtaler** mellem to brugere
- **Realtime-beskeder** med notifikationer
- **Samtalehistorik** der bevares på tværs af sessioner
- **Online-status** indikatorer for at se, hvornår nogen er tilgængelig
- **Registrering af ulæste beskeder** for at holde styr på nye beskeder

### Starte en direkte besked

Der er flere måder at starte en DM-samtale på:

**Fra en brugerprofil:**
1. Besøg brugerens profilside
2. Klik på fanen "Direkte Beskeder" eller knappen "Send besked"
3. Skriv din besked og send

**Fra en kommentar:**
Nogle implementeringer tillader, at du kan klikke på en brugers navn eller avatar for hurtigt at få adgang til deres profil og beskedmuligheder.

**Direkte link:**
Hvis du har en samtale-id eller et DM-omdirigeringslink, kan du få direkte adgang til samtaler.

### Få adgang til dine beskeder

For at se alle dine direkte beskedesamtaler:

1. Besøg din profils side
2. Klik på fanen "Direkte Beskeder"
3. Se en liste over alle dine samtaler

Hver samtale viser:
- Den anden deltageres avatar og navn
- Deres online/offline-status (grøn indikator når online)
- En forhåndsvisning af den sidste besked
- Antal ulæste beskeder (hvis nogen)
- Tidsstempel for sidste aktivitet

### Samtaleblik

Når du åbner en samtale, vil du se:

- **Fuld beskedhistorik** - Alle beskeder mellem dig og den anden bruger
- **Realtime-opdateringer** - Nye beskeder vises øjeblikkeligt via WebSocket
- **Online-status** - Se om den anden person er online lige nu
- **Tidsstempler på beskeder** - Hvornår hver besked blev sendt
- **Sammensætning af beskeder** - Tekstfelt til at skrive og sende nye beskeder

### Beskednotifikationer

Du vil modtage notifikationer for nye direkte beskeder:

- **Notifikationer i appen** - Badge-tæller på din profil
- **E-mailnotifikationer** - Afhængig af dine notifikationsindstillinger
- **Realtime-advarsler** - Øjeblikkelige notifikationer når du er logget ind

Administrer notifikationspræferencer i dine [Kontoindstillinger](https://fastcomments.com/auth/my-account/edit-notifications).

### Privatliv og blokering

**Deaktivere direkte beskeder:**
Hvis du ikke vil modtage DMs, kan du slå dem helt fra:
1. Gå til Privatlivsindstillinger på din profil
2. Aktivér "Deaktiver Direkte Beskeder"
3. DM-muligheden vil blive skjult fra din profil

For flere detaljer, se [Privatlivsindstillinger](/guide-user-profiles.html#privacy-settings).

**Blokere brugere:**
Hvis nogen chikanerer dig via DM:
1. Besøg deres profil
2. Bloker brugeren
3. De vil ikke længere kunne sende dig beskeder

Blokerede brugere kan ikke:
- Sende dig direkte beskeder
- Se din aktivitet
- Kommentere på din profil
- Interagere med dig nogen steder på FastComments

### Administrere samtaler

**Skjule samtaler:**
Du kan skjule samtaler, som du ikke længere ønsker at se i din samtaleliste:
1. Åbn fanen Direkte Beskeder
2. Find samtalen du vil skjule
3. Vælg skjul/arkiver-indstillingen

Skjulte samtaler vises ikke i din hovedliste, men kan tilgås, hvis den anden person sender en ny besked.

**Markere som læst:**
Beskeder markeres automatisk som læst, når du ser dem. Du kan også manuelt markere hele samtaler som læst:
1. Åbn fanen Direkte Beskeder
2. Vælg en samtale
3. Vælg "Markér som læst"

Dette nulstiller den ulæste tæller for den samtale.

### Bedste praksis

**Hvornår du skal bruge Direkte Beskeder:**
- Stille opfølgende spørgsmål privat
- Takke nogen for hjælpsom rådgivning
- Diskutere uden for emne uden at rode de offentlige kommentarer
- Koordinere med andre fællesskabsmedlemmer
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
- Vær forsigtig med mistænkelige links eller anmodninger
- Brug blokering, hvis nogen gør dig utilpas

### Begrænsninger og bemærkninger

**Hvem kan du sende besked til:**
- Enhver FastComments-bruger, der ikke har deaktiveret DMs
- Brugere, der ikke har blokeret dig
- Brugere på tværs af alle FastComments-fællesskaber

**Beskedindhold:**
- Tekstbeskeder understøttes
- Beskeder følger de samme indholdspolitikker som kommentarer
- Upassende indhold kan rapporteres

**Samtaleomfang:**
- DM-samtaler er private mellem to personer
- Ingen gruppesamtaler (pt. kun én-til-én)
- Samtalehistorik bevares på ubestemt tid

### Fejlfinding

**Kan ikke sende en besked?**
Den anden bruger kan have:
- Deaktiveret direkte beskeder i deres privatlivsindstillinger
- Blokeret dig
- Slettet deres konto

**Modtager du ikke notifikationer?**
Tjek dine notifikationsindstillinger i [Kontoindstillinger](https://fastcomments.com/auth/my-account/edit-notifications) for at sikre, at DM-notifikationer er aktiveret.

**Beskeder bliver ikke sendt?**
- Tjek din internetforbindelse
- Opdater siden og prøv igen
- Sørg for, at du ikke er blevet blokeret
- Kontakt support, hvis problemet fortsætter