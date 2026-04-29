---
Sproži se, ko je komentar pripet.

### Kontekst, ki ga prejme agent

- Pripet komentar.
- Neobvezen kontekst niti / zgodovina uporabnika / kontekst strani, kot je konfigurirano.

### Kdo sproži to

- Moderator, ki klikne dejanje pripenjanja na strani za moderiranje ali v pripomočku za komentarje.
- Agent, ki kliče [`pin_comment`](#tools-overview).

Preprečevanje zank: dogodki pripenjanja, ki izvirajo od agenta, ne sprožijo nobenih agentnih sprožilcev. Pripenjanje, izvedeno s strani agenta, prekine vso agentno razpošiljanje za ta dogodek, ne le razpošiljanje izvirnega agenta.

### Opomba o paru

Dogodki pripenjanja in odpenjanja so ločeni sprožilci. Naročite se na oba, če želite simetrično vedenje. Oglejte si [Sprožilec: Komentar odpet](#trigger-comment-unpin).

---