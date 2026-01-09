---
Comme tout système distribué, FastComments a besoin d'un moyen pour verrouiller des ressources et des procédures. Ces verrous peuvent être surveillés via le point de terminaison `/locks-in-progress`.

[Par exemple, voici le point de terminaison de notre shard US](https://fastcomments.com/locks-in-progress).

Cela peut être utile pour comprendre pourquoi le système est bloqué ou en surcharge. Si, par exemple, un SRE veut voir pourquoi le système subit une charge CPU élevée, il/elle pourrait
consulter ce point de terminaison pour obtenir le nom du cron défaillant.

---