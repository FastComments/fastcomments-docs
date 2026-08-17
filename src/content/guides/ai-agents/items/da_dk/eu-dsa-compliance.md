FastComments håndhæver artikel 17 i EU's Digital Services Act for lejere i EU-regionen: **fuldautomatisk bruger suspendering er ikke tilladt**.

### Hvad det betyder i praksis

Når din lejer er i EU-regionen, på agentens redigeringsformular:

- Afkrydsningsfeltet **Approvals** for `ban_user` er **låst til** og kan ikke fjernes.
- Etiketten lyder: "EU DSA Artikel 17: bruger suspenderinger kræver menneskelig gennemgang. 'Ban a user' er låst til og kan ikke fuldt automatiseres i EU-regionen."
- Et værktøjstip i godkendelseskolonnen lyder: "Låst til af EU DSA Artikel 17 - fuldautomatiske bans er ikke tilladt i EU-regionen."

Uanset hvad du ellers konfigurerer, går hver `ban_user`-opkald fra enhver agent på en EU-region lejer til [approvals inbox](#approval-workflow) for menneskelig gennemgang. Ban'en sker ikke, før en person godkender den.

### Hvorfor dette håndhæves på platformniveau, ikke på promptniveau

Systemprompter kan ignoreres eller omgås af en tilstrækkeligt fejlagtig model. Overholdelse af artikel 17 er for vigtig til at stole på modellens gode opførsel; det skal være en hård server-side gate, som værktøjsdistributøren selv håndhæver. Det er præcis, hvad vi gør.

### Hvad der går gennem godkendelse, og hvad der ikke gør

- **`ban_user`**: altid gatekørt i EU. Inkluderer:
  - Synlige bans (`shadowBan: false`).
  - Skyggebans (`shadowBan: true`).
  - Bans med `deleteAllUsersComments: true`.
  - Bans med `banIP: true`.
- Alle ban-variationer lander i godkendelsesindbakken med agentens begrundelse og sikkerhed; en person godkender eller afviser.

De andre agentværktøjer (`mark_comment_spam`, `warn_user`, `lock_comment` osv.) er **ikke** påvirket af artikel 17. Du kan stadig automatisere dem. Artikel 17 handler specifikt om bruger suspenderinger.

### Hvad med ikke-EU lejere

Låsen gælder ikke uden for EU-regionen. Du kan vælge at gate `ban_user` bag godkendelse alligevel - vi anbefaler kraftigt dette i de første uger af enhver moderationsagents liv - men det håndhæves ikke.

### Skyggebans

Skyggebans tæller som suspenderinger for artikel 17-formål (brugeren kan poste, men deres indhold er skjult). De er gatekørt identisk med synlige bans.

### Regiondetektion

Regionen bestemmes på procesniveau af `REGION`-miljøvariablen på FastComments-implementeringen (læst af `isEURegion()` i `models/constants.ts`). Der er ingen region-felt per lejer - låsen gælder for hver lejer på en EU-implementeret instans. Hvis du migrerer dine data fra en ikke-EU-implementering til en EU-implementering, træder låsen i kraft for alle lejere på den instans.

### Hvad hvis alle anmeldere er utilgængelige

Godkendelsen vil forblive i indbakken indtil den er besluttet. Den udløber automatisk 90 dage efter oprettelse. Der er ingen "ingen reviewer tilgængelig, fal gennem til automatiseret beslutning"-sti - det ville undergrave pointen med artikel 17.

Hvis dit fællesskab er så højvolumen, at EU-bans ikke kan gennemgås inden for rimelig tid, overvej:

- Tilføj flere anmeldere (se [Approval Notifications](#approval-notifications)).
- Skift agenten til at bruge [`warn_user`](#tool-warn-user) mere aggressivt, da advarsler ikke er underlagt artikel 17.
- Sænk agentens trang til at banne ved at stramme [community guidelines](#community-guidelines) eller [initial prompt](#personality-prompt) op.

### Se også

- [Tool: ban_user](#tool-ban-user) for hvad `ban_user` gør og de destruktive muligheder bag ekstra opt-ins.
- [Approval Workflow](#approval-workflow) for den fulde godkendelseslivscyklus.