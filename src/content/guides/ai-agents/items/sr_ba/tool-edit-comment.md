Alat za uređivanje omogućava agentu da zamijeni tekst postojećeg komentara. On je destruktivan na način na koji većina drugih alata za moderaciju nije: prepisuje sadržaj koji je napisao korisnik. Koristite ga samo za uske, jasno definirane slučajeve.

### Šta radi

Agent prosljeđuje ID komentara i zamjenski sadržaj. Platforma upisuje novi tekst u komentar i evidentira unos `TextChanged` u revizijskom dnevniku komentara koji bilježi i stari tekst i novi tekst. Original nikad nije izgubljen — moderatori mogu pročitati šta je komentar sadržavao prije nego što ga je agent uredio.

Zamjena prolazi kroz isti pipeline renderovanja kao i ljudska izmjena: maskiranje psovki, parsiranje spominjanja, ekstrakcija hashtagova i rukovanje linkovima/slikama sve se ponašaju upravo kao da je originalni autor poslao novi tekst.

### Opseg

Kao i svaki alat koji mijenja komentare, Edit je ograničen na allowlist okidača — agent može uređivati samo komentar na kojem je okidač aktiviran, njegovog roditelja, ili neki drugi komentar koji je u opsegu istog konteksta okidača. Pokušaj ubacivanja prompta da "edit comment XYZ" gdje XYZ nije povezan biće odbijen na serverskoj strani prije nego što se izvršitelj pokrene.

### Petlje

Kada agent uredi komentar, platforma aktivira `COMMENT_EDIT` okidač kao što bi to učinila za ljudsku izmjenu, ali **sprječava slanje drugim agentima**. Ovo sprečava da se dva agenta koja slušaju `COMMENT_EDIT` međusobno ping-pongaju na osnovu svojih izmjena.

### Kada ga dozvoliti

Za agente koji rade redakciju/skrivanje PII (osobni identifikacijski podaci), ili za agente koji sami uređuju sažetke/izvode. Većina moderacijskih agenata **ne** treba ovaj alat - mark-spam, warn, i ban pokrivaju tipični životni ciklus.

### Odobrenja

**Snažno razmotrite stavljanje iza procesa odobrenja**, posebno dok gradite povjerenje u agenta. Agent koji preformuliše korisnikove riječi je akcija koju će zajednica primijetiti i na koju će reagirati, i reputacijski je teže „poništiti“ nego brisanje.

### Vidi također

- [Trigger: Comment Edited](#trigger-comment-edit) - okidač koji se aktivira kada se tekst komentara promijeni.
- [Approval Workflow](#approval-workflow) - kako staviti alat iza ljudske provjere.