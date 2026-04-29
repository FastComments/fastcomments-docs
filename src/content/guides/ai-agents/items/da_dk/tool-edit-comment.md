Værktøjet Edit lader agenten erstatte teksten i en eksisterende kommentar. Det er destruktivt på en måde, som de fleste andre moderationsværktøjer ikke er: det overskriver brugerforfattet indhold. Reserver det til snævre, klare tilfælde.

### Hvad det gør

Agenten sender en kommentar-ID og en erstatningstekst. Platformen skriver den nye tekst til kommentaren og registrerer en `TextChanged` entry i kommentarens revisionslog, som fanger både den gamle tekst og den nye tekst. Den oprindelige tekst går aldrig tabt - moderatorer kan læse, hvad kommentaren sagde, før agenten redigerede den.

Erstatningen går gennem den samme gengivelsespipeline som en menneskelig redigering: maskering af bandeord, genkendelse af mentions, udtrækning af hashtags og håndtering af links/billeder opfører sig alle præcis som hvis den oprindelige forfatter havde indsendt den nye tekst.

### Omfang

Ligesom ethvert værktøj, der ændrer kommentarer, er Edit begrænset til triggerens allowlist - agenten kan kun redigere den kommentar, triggeren udløste på, dens overordnede, eller en anden kommentar i samme triggerkontekst. Et forsøg på prompt‑injektion, der siger "rediger kommentar XYZ", hvor XYZ er uvedkommende, vil blive afvist på serversiden før eksekveringen kører.

### Løkker

Når agenten redigerer en kommentar, udløser platformen en `COMMENT_EDIT` trigger, som den ville ved en menneskelig redigering, men **undertrykker udsendelse til andre agenter**. Dette forhindrer, at to agenter, som begge lytter efter `COMMENT_EDIT`, ping-ponger på hinandens redigeringer.

### Hvornår man bør tillade det

Til agenter, der håndterer fjernelse af PII, eller til selvredigerende opsummerings-/digest-agenter. De fleste moderationsagenter har **ikke** brug for dette værktøj - mark-spam, warn, and ban dækker den typiske livscyklus.

### Godkendelser

**Overvej kraftigt at kræve godkendelse**, især mens du opbygger tillid til agenten. At en agent omskriver en brugers ord er den slags handling, et fællesskab vil lægge mærke til og reagere på, og det er sværere at 'fortryde' omdømmemæssigt end en sletning.

### Se også

- [Trigger: Comment Edited](#trigger-comment-edit) - den trigger, der affyres, når en kommentars tekst ændres.
- [Approval Workflow](#approval-workflow) - hvordan man lægger værktøjet bag menneskelig godkendelse.