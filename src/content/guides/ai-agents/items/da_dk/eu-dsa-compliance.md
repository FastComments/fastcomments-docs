---
FastComments håndhæver Artikel 17 i EU's Digital Services Act for lejere i EU-regionen: **fuldautomatiske bruger-suspensioner er ikke tilladt**.

### Hvad det betyder i praksis

Når din lejer er i EU-regionen, på agentens redigeringsformular:

- Afkrydsningsfeltet **Approvals** for `ban_user` er **låst til** og kan ikke fjernes.
- Etiketteksten lyder: "EU DSA Article 17: user suspensions require human review. 'Ban a user' is locked on and cannot be fully automated in the EU region."
- En tooltip i godkendelseskolonnen lyder: "Locked on by EU DSA Article 17 - fully-automated bans are not permitted in the EU region."

Uanset hvad du ellers konfigurerer, går hver `ban_user`-anmodning fra en agent på en lejer i EU-regionen til [godkendelsesindbakken](#approval-workflow) til menneskelig gennemgang. Suspenderingen sker ikke, før en person godkender den.

### Hvorfor dette håndhæves på platformniveau og ikke i prompten

Systemprompter kan ignoreres eller omgås af en tilstrækkeligt fejlagtig model. Overholdelse af Artikel 17 er for vigtig til at stole på modellens gode opførsel; det skal være en hård server-side port, som værktøjsdispatcheren selv håndhæver. Det er det, vi gør.

### Hvad der går igennem godkendelse, og hvad der ikke gør

- **`ban_user`**: altid begrænset i EU. Inklusive:
  - Synlige suspensioner (`shadowBan: false`).
  - Skygge-suspensioner (`shadowBan: true`).
  - Suspensioner med `deleteAllUsersComments: true`.
  - Suspensioner med `banIP: true`.
- Alle varianter af suspensioner havner i godkendelsesindbakken med agentens begrundelse og konfidens; en person godkender eller afviser.

De andre agentværktøjer (`mark_comment_spam`, `warn_user`, `lock_comment`, etc.) er **ikke** påvirket af Artikel 17. Du kan stadig automatisere dem. Artikel 17 handler specifikt om bruger-suspensioner.

### Hvad med lejere uden for EU

Låsen gælder ikke uden for EU-regionen. Du kan vælge at placere `ban_user` bag en godkendelse alligevel — vi anbefaler kraftigt at gøre det i de første uger af en moderation agents levetid — men det håndhæves ikke.

### Skygge-suspensioner

Skygge-suspensioner tæller som suspensioner i henhold til Artikel 17 (brugeren kan poste, men deres indhold er skjult). De er begrænset på samme måde som synlige suspensioner.

### Regionsdetektion

Regionen bestemmes på procesniveau af miljøvariablen `REGION` på FastComments-implementeringen (læses af `isEURegion()` i `models/constants.ts`). Der er ikke et regionsfelt per lejer - låsen gælder for alle lejere på en EU-deployeret instans. Hvis du migrerer dine data fra en ikke-EU-implementering til en EU-implementering, træder låsen i kraft for alle lejere på den instans.

### Hvad hvis alle gennemgåere er utilgængelige

Godkendelsen bliver i indbakken, indtil der træffes en afgørelse. Den udløber automatisk 90 dage efter oprettelse. Der er ingen "ingen gennemgår tilgængelig, fald tilbage til automatisk beslutning"-sti — det ville underminere formålet med Artikel 17.

Hvis dit fællesskab er så højt-volumen, at EU-suspensioner ikke kan gennemgås inden for rimelig tid, overvej:

- At tilføje flere gennemgåere (se [Approval Notifications](#approval-notifications)).
- At skifte agenten til at bruge `warn_user` mere aggressivt, da advarsler ikke er omfattet af Artikel 17.
- At sænke agentens trang til at suspendere ved at stramme [fællesskabets retningslinjer](#community-guidelines) eller [indledende prompt](#personality-prompt).

### Se også

- [Værktøj: ban_user](#tool-ban-user) for hvad `ban_user` gør og de destruktive muligheder bag ekstra opt-ins.
- [Approval Workflow](#approval-workflow) for hele godkendelseslivscyklussen.

---