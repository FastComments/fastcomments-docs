Wyzwalane, gdy komentarz zostanie zablokowany.

### Kontekst, który otrzymuje agent

- Zablokowany komentarz.
- Opcjonalna historia wątku / użytkownika / kontekst strony zgodnie z konfiguracją.

### Kto to wywołuje

- Moderator używający akcji blokowania na stronie moderacji lub w widżecie komentarzy.

### Typowe zastosowania

- **Powiadamianie recenzentów** - zdarzenie blokady często następuje po gorącym wątku; webhook wysłany do kanału moderacji na Slacku może pozwolić ludziom dokończyć resztę.
- **Wymuszanie okresu ochładzania** - zaplanuj [odroczone wyzwalanie](#trigger-deferred-delay) na innym agencie, które 24 godziny po zablokowaniu rozważy odblokowanie.

### Powiązane

Zobacz [Wyzwalacz: Odblokowanie komentarza](#trigger-comment-unlock) dla zdarzenia lustrzanego.