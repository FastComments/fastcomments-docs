FastComments supporta i webhook solo per la risorsa Comment.

Supportiamo i webhook per la creazione, la rimozione e l'aggiornamento dei commenti.

Ciascuno di questi è considerato un evento separato nel nostro sistema e, come tale, ha semantiche e strutture diverse per gli eventi webhook.

Un numero qualsiasi di endpoint può iscriversi allo stesso evento: è possibile configurare un webhook per dominio nella dashboard, e ulteriori iscrizioni possono essere create tramite l'API (vedi Gestione dei webhook tramite l'API).