---
Come qualsiasi sistema distribuito, FastComments necessita di un meccanismo per bloccare risorse e procedure. Questi blocchi possono essere monitorati tramite l'endpoint `/locks-in-progress`.

[Ad esempio, ecco l'endpoint sul nostro shard US](https://fastcomments.com/locks-in-progress).

Questo può essere utile per capire perché il sistema è bloccato o sotto carico. Se magari un SRE vuole capire perché il sistema sta riscontrando un elevato utilizzo della CPU, potrebbe controllare questo endpoint per ottenere il nome del cron che si comporta in modo anomalo.

---