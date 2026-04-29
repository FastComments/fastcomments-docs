Alat Edit omogućuje agentu zamjenu teksta postojećeg komentara. To je destruktivno na način na koji većina drugih alata za moderiranje nije: prepisuje sadržaj koji je napisao korisnik. Koristite ga samo u uskim, jasno definiranim slučajevima.

### Što radi

Agent prosljeđuje ID komentara i zamjenski sadržaj. Platforma upisuje novi tekst u komentar i bilježi unos `TextChanged` u zapisnik revizije komentara koji bilježi i stari i novi tekst. Izvorni sadržaj nikada nije izgubljen — moderatori mogu pročitati što je komentar sadržavao prije nego što ga je agent uredio.

Zamjena prolazi kroz isti proces renderiranja kao i ljudska izmjena: maskiranje psovki, parsiranje spominjanja, ekstrakcija hashtaga i rukovanje linkovima/slikama ponašaju se točno kao da je izvorni autor poslao novi tekst.

### Opseg

Kao i svaki alat koji mijenja komentare, Edit je ograničen popisom dopuštenih okidača — agent može urediti samo komentar na koji se okidač aktivirao, njegovog roditelja ili neki drugi komentar unutar istog konteksta okidača. Pokušaj prompt-injectiona da "edit comment XYZ" gdje je XYZ nepovezan, bit će odbijen na strani poslužitelja prije nego što izvršitelj pokrene akciju.

### Petlje

Kada agent uredi komentar, platforma pokreće okidač `COMMENT_EDIT` kao što bi to učinila za ljudsku izmjenu, ali **sprječava slanje drugim agentima**. To sprječava da se dva agenta koja slušaju `COMMENT_EDIT` međusobno ping-pongaju na svojim izmjenama.

### Kada dopustiti

Za agente koji se bave uklanjanjem PII, ili za agente koji sami uređuju i sažimaju/digestiraju sadržaj. Većina agenata za moderaciju **ne treba** ovaj alat - mark-spam, warn i ban pokrivaju tipični životni ciklus.

### Odobrenja

**Snažno razmotrite postavljanje ovog alata iza procesa odobrenja**, osobito dok gradite povjerenje u agenta. Agent koji prepisuje korisnikove riječi vrsta je akcije koju će zajednica primijetiti i na koju će reagirati, i reputacijski ju je teže "poništiti" nego brisanje.

### Vidi također

- [Trigger: Comment Edited](#trigger-comment-edit) - okidač koji se aktivira kada se tekst komentara promijeni.
- [Approval Workflow](#approval-workflow) - kako staviti alat iza ljudske recenzije.