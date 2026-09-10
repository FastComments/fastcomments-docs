## Sprožilci

Sprožilci zaženejo Zap, ko se v FastComments zgodi nekaj. Vsi trije so takojšnji: FastComments pošlje dogodek Zapierju prek webhooka takoj, ko se zgodi. Nič ne poizveduje vašega računa in pri čakanju ne porabite API kreditov.

| Sprožilec | Ko se sproži |
|-----------|--------------|
| Nov komentar | Komentar je objavljen. Privzeto se sprožijo le odobreni, ne‑spam komentarji. |
| Posodobljen komentar | Komentar je urejen, odobren, glasovan, pripet, zaklenjen ali na drug način spremenjen. |
| Izbrisan komentar | Komentar je izbrisan. |

Vsak sprožilec vrne celoten komentar: id, URL strani in URL ID, ime in e‑mail komentatorja, besedilo komentarja kot markdown in kot HTML, število glasov, oznake odobritve in spama, jezikovno nastavitev, domeno in morebitne omembe. Polja se ujemajo s payloadom webhooka, dokumentiranim pod Webhooks, Data Structures.

## Možnosti

**Domena.** Vsak sprožilec ima neobvezen filter domene, ki prikazuje domene, nastavljene v vašem računu. Pustite prazno, da prejmete dogodke iz vseh domen.

**Vključi neodobrene in spam komentarje.** Le pri sprožilcu Nov komentar. Komentarji, ki so zadržani za moderacijo ali označeni kot spam, so privzeto izpuščeni. Ko je tak komentar kasneje odobren, se sproži sprožilec Posodobljen komentar, zato Zap, ki naj bi reagiral na vsak komentar, ki postane viden, uporablja Posodobljen komentar s filtrom na polju odobren.

## Kako deluje dostava

Vklop Zap-a ustvari naročnino na webhook v vašem računu, vidno na strani Webhooks s virom **API**. Izklop Zap-a jo odstrani. Zapierjeve lastne omejitve veljajo za število dogodkov, ki jih sprejme na minuto; FastComments ponovi pošiljanje, ki ne uspe, z naraščajočim zamikom, in onemogoči naročnino, ki ne uspe šest dni. Onemogočeno naročnino lahko ponovno omogočite na strani Webhooks ali preprosto izklopite in ponovno vklopite Zap, da ustvarite novo.

Račun lahko vsebuje največ 50 API naročnin. Vsak Zap, ki uporablja sprožilec FastComments, uporabi eno.

---