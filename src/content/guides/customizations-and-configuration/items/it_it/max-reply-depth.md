[related-parameter-start name = 'maxReplyDepth'; type = 'number'; related-parameter-end]

Per impostazione predefinita, FastComments consente un annidamento illimitato delle risposte, creando una struttura a thread in cui gli utenti possono rispondere indefinitamente alle risposte.

L'opzione maxReplyDepth ti consente di limitare la profondità massima dei thread di risposte. Quando viene raggiunta la profondità massima, gli utenti non vedranno più il pulsante di risposta sui commenti a quel livello.

[code-example-start config = {maxReplyDepth: 2}; linesToHighlight = [6]; title = 'Limiting Reply Depth to 2 Levels'; code-example-end]

Con maxReplyDepth impostato su 2:
- Gli utenti possono commentare al livello principale (profondità 0)
- Gli utenti possono rispondere ai commenti di livello principale (profondità 1)
- Gli utenti possono rispondere a quelle risposte (profondità 2)
- Non sono consentite ulteriori risposte oltre la profondità 2

Impostarlo su 1 consentirebbe solo risposte ai commenti di livello principale, creando una struttura di discussione più piatta.

Impostare maxReplyDepth a 0 disabiliterebbe tutte le risposte, permettendo solo commenti di livello principale. Se non specificato, le risposte possono essere annidate senza limiti.
---