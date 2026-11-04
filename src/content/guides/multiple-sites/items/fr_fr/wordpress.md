Chaque site WordPress a besoin de son propre compte FastComments ou de son propre tenant. WordPress numérote les articles séparément sur chaque site, donc l'article 5 sur un site et l'article 5 sur un autre partageraient le même fil de commentaires s'ils étaient tous deux connectés au même compte. Pour cette raison, la configuration du plugin refuse de connecter un site lorsque le compte a déjà un autre site connecté.

[Vous pouvez voir vos intégrations actives ici.](https://fastcomments.com/auth/my-account/manage-data/integrations)

Pour gérer plusieurs sites WordPress, créez un [tenant](https://fastcomments.com/auth/my-account/tenants) pour chacun, basculez dessus, puis connectez le site depuis la page de configuration du plugin. Les tenants sont inclus dans les plans avec marque blanche. Vous pouvez également créer un compte FastComments distinct pour chaque site, en utilisant une adresse e‑mail différente pour chacun.

Si un site a été déplacé, par exemple du staging à la production, désactivez le plugin sur l'ancien site et retirez l'ancien site de vos intégrations actives. Connectez ensuite le nouveau. Les commentaires déjà synchronisés depuis l'ancien site restent dans le compte, et ils s'affichent sur tout article ayant le même ID sur le nouveau site.

Vous pouvez utiliser le même compte FastComments avec une seule installation WordPress qui s'étend sur plusieurs domaines et possède le même contenu sur ces domaines.