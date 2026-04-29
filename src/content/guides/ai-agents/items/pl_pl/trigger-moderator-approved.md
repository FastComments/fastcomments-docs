Wyzwalane, gdy moderator zatwierdzi komentarz.

### Kontekst, który otrzymuje agent

- Nowo zatwierdzony komentarz.
- **identyfikator użytkownika wyzwalającego** - moderator, który zatwierdził.
- Opcjonalna historia wątku / użytkownika / kontekst strony zgodnie z konfiguracją.

### Kto to wywołuje

Akcja wykonywana przez ludzkiego moderatora.

### Ważne

- Komentarz „zatwierdzony” to w terminologii FastComments komentarz **widoczny**. Zobacz [Jak działają zatwierdzenia](/guide-moderation.html#moderation-approvals) w przewodniku moderacji, aby poznać różnicę między zatwierdzonym/niezatwierdzonym a zrecenzowanym/niezrecenzowanym.
- Wyzwalacz uruchamia się przy **przejściach** zatwierdzenia: komentarz przechodzący z niezatwierdzonego na zatwierdzony go uruchamia; komentarz, który był już zatwierdzony i został ponownie zapisany, nie uruchamia go.
- W przypadku tenantów, w których komentarze domyślnie są automatycznie zatwierdzane, ten wyzwalacz uruchamia się tylko wtedy, gdy moderator wyraźnie ponownie zatwierdzi wcześniej ukryty komentarz.

### Typowe zastosowania

- **Powitanie / zaangażowanie** - agent może odpowiedzieć osobom komentującym po raz pierwszy w momencie, gdy moderator je zatwierdzi, zamiast robić to w momencie publikacji.
- **Koordynacja między agentami** - jeśli inny agent oznaczył komentarz do przeglądu, zatwierdzenie jest sygnałem, że przegląd przez człowieka został zakończony.
- **Rejestr audytu** za pośrednictwem [Webhooks](#webhooks-overview).