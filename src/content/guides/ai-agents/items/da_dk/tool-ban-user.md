Ban-værktøjet er den mest afgørende handling, en agent kan kalde. Det udelukker en bruger fra dit community i en fast varighed og med nogle få valgfrie indstillinger.

### Hvad det gør

Agenten vælger en af seks varigheder:

- En time
- En dag
- En uge
- En måned
- Seks måneder
- Et år

Den vælger også mellem en **synlig udelukkelse** (brugeren ser en tydelig udelukkelsesbesked og kan anke) og en **skjult udelukkelse** (brugeren kan fortsætte med at poste, men deres indhold er skjult for andre brugere). Platformens instruktioner beder agenten om at foretrække synlige udelukkelser for førstegangs- eller gråzone-tilfælde, og skjulte udelukkelser for klart ondsindede gentagne overtrædere.

### De to destruktive underindstillinger

To ekstra indstillinger er **skjult for agenten som standard**. For at aktivere en af dem, sæt flueben i den tilsvarende afkrydsningsboks i sektionen **Indstillinger for udelukkelse** på agentens redigeringsformular:

- **Allow deleting all of the user's comments.** Når den er aktiveret, kan agenten vælge også at slette alle kommentarer, den bandlyste bruger nogensinde har postet i din tenant. Forbeholdes klart spam, doxxing eller koordineret misbrug, hvor det eksisterende indhold ikke har værdi. **Destruktiv og uigenkaldelig.**
- **Allow banning by IP.** Når den er aktiveret, kan agenten vælge også at udelukke den IP-adresse, kommentaren blev postet fra. Nyttigt mod omgåelse ved alternative konti. **Undgå for delte IP-adresser** (virksomhed, skole, mobiludbydere) - uskyldige brugere på samme netværk vil blive blokeret.

Platformen håndhæver det også serverside: selv hvis agenten går rogue og forsøger at kalde optionen, afvises anmodningen, medmindre du har tilvalgt den.

### Optrapningspolitik

Før udelukkelse instruerer platformen agenten til at:

1. Søge i [agent memory](#agent-memory-system) efter tidligere advarsler eller notater om brugeren.
2. Foretrække at [advare](#tool-warn-user) brugeren fremfor at udelukke ved første forseelse.
3. Kun springe advarselssteppet over ved klart grove tilfælde (ulovligt indhold, doxxing, koordineret spam) - og forklare hvorfor i sin begrundelse.

Denne politik er i agentens instrukser, ikke en hård serverside-regel, hvilket er grunden til, at det **kraftigt anbefales at kræve godkendelse for udelukkelser**.

### EU-region: menneskelig godkendelse påkrævet

I EU-regionen er dette værktøj **låst til godkendelse** af artikel 17 i Digital Services Act. Enhver udelukkelse fra en agent på en tenant i EU-regionen lander i [approvals inbox](#approval-workflow) til menneskelig gennemgang. Se [EU DSA Article 17 Compliance](#eu-dsa-compliance).

### Anbefalinger

- Kræv godkendelse overalt i mindst den første måned.
- Gate altid **delete-all-comments**-muligheden, hvis du aktiverer den - den er irreversible.
- Overvej at kræve godkendelse for **IP ban**-muligheden, selv efter agenten har opbygget tillid - prisen ved en IP-ban på et delt netværk viser sig ikke i agentens kørselslog.

### Se også

- [Banning Users](/guide-moderation.html#banning-users) og [Banning Users With Wildcards](/guide-moderation.html#banning-users-wildcards) i moderation-guiden for hvordan udelukkelser fungerer på tværs af platformen.
- [Warn user](#tool-warn-user) - det mildere optrapningstrin.