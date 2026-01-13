---
Comme tout système distribué, FastComments a besoin d'un moyen de verrouiller les ressources et les procédures. Ces verrous peuvent être surveillés via le point de terminaison `/locks-in-progress`.

[Par exemple, voici le point de terminaison sur notre shard américain](https://fastcomments.com/locks-in-progress).

Cela peut être utile pour voir pourquoi le système est bloqué, ou soumis à une charge. Si un SRE veut savoir pourquoi le système connaît une utilisation élevée du processeur, il pourrait
vérifier ce point de terminaison pour obtenir le nom du cron qui se comporte mal.

---