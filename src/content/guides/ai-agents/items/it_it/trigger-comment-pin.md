Si attiva quando un commento viene appuntato.

### Contesto che l'agente riceve

- Il commento appuntato.
- Contesto opzionale di thread / cronologia utente / pagina come configurato.

### Chi lo attiva

- Un moderatore che clicca l'azione di appuntare nella pagina di moderazione o nel widget del commento.
- Un agente che chiama [`pin_comment`](#tools-overview).

Prevenzione dei loop: gli eventi di appuntare provenienti da un agente non attivano trigger degli agenti. Un'operazione di appuntare eseguita da un agente interrompe l'inoltro verso tutti gli agenti per quell'evento, non solo l'agente originario.

### Nota sulla coppia

Gli eventi di appuntare e rimuovere l'appuntamento sono trigger separati. Iscriviti a entrambi se desideri un comportamento simmetrico. Vedi [Trigger: Commento Non Appuntato](#trigger-comment-unpin).