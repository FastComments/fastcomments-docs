Alat Edit omogućava agentu da zameni tekst postojećeg komentara. To je destruktivno na način na koji većina drugih alatki za moderaciju nije: prepisuje sadržaj koji su napisali korisnici. Koristite ga samo u usko ograničenim, jasno definisanim slučajevima.

### Šta radi

Agent prosleđuje ID komentara i novi sadržaj. Platforma upisuje novi tekst u komentar i beleži `TextChanged` unos u audit logu komentara koji beleži i stari i novi tekst. Original nikada nije izgubljen — moderatori mogu pročitati šta je komentar sadržao pre nego što ga je agent izmenio.

Zamenski tekst prolazi kroz isti proces renderovanja kao i ljudska izmena: maskiranje psovki, parsiranje pomena, izdvajanje heštegova i rukovanje linkovima/slikama rade tačno isto kao da je originalni autor poslao novi tekst.

### Opseg

Kao i svaki alat koji menja komentare, Edit je ograničen na listu dozvoljenih okidača — agent može da izmeni samo komentar na kojem je okidač pokrenut, njegov roditelj ili neki drugi komentar u okviru istog konteksta okidača. Pokušaj prompt-injectiona da "edit comment XYZ" gde je XYZ nepovezan biće odbijen na serverskoj strani pre nego što izvršilac počne.

### Petlje

Kada agent izmeni komentar, platforma pokreće `COMMENT_EDIT` okidač kao i kod ljudske izmene, ali **suzbija slanje obaveštenja drugim agentima**. Ovo sprečava da se dva agenta koja slušaju `COMMENT_EDIT` ping-ponguju međusobnim izmenama.

### Kada ga dozvoliti

Za agente koji rade redakciju PII, ili za agente koji samostalno uređuju sažetke/digeste. Većini agenata za moderaciju ovaj alat **nije** potreban — mark-spam, warn i ban pokrivaju tipični životni ciklus.

### Odobrenja

**Snažno razmotrite ograničavanje iza odobrenja**, posebno dok gradite poverenje u agenta. Agent koji prepravlja reči korisnika je vrsta akcije koju će zajednica primetiti i na koju će reagovati, i reputaciono je teže "undo" nego brisanje.

### Pogledajte takođe

- [Trigger: Comment Edited](#trigger-comment-edit) - okidač koji se pokreće kada se tekst komentara promeni.
- [Approval Workflow](#approval-workflow) - kako ograničiti alat iza ljudske revizije.