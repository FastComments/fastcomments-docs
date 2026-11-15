Une facture Flex est une liste de lignes, une par type d'utilisation, chacune indiquant la quantité et le tarif, par exemple `213,000 Page Loads. ($1.00 per 100,000 Page Loads)`. Les tarifs figurent sur la [page de tarification](https://fastcomments.com/traffic-pricing) et sur votre page [Billing Info](https://fastcomments.com/auth/my-account/billing-info). Cette section explique la signification de chaque quantité.

### Compté sur le mois

Ces compteurs sont réinitialisés chaque mois et sont comptés du premier au dernier jour du mois calendaire (UTC) :

- **Page Loads** : le nombre de fois où un widget de commentaires ou un fil a été chargé sur vos pages. Chaque chargement du widget de commentaires compte, quel que soit le nombre de commentaires affichés. Il n'y a aucun frais par fil de commentaires ni par page.
- **API Credits** : les appels à l'API REST avec votre clé API.
- **Small Widgets Credits** : les chargements des petits widgets, tels que les compteurs de commentaires et les badges.
- **LLM Tokens** : les jetons utilisés par les [AI Agents](/guide-ai-agents.html) et les autres fonctionnalités LLM.

### Compté à la fin du mois

Il s'agit d'un instantané pris lors de la création de la facture :

- **Comments Stored** : le nombre total de commentaires dans votre compte, et pas seulement ceux ajoutés ce mois-ci. La suppression de commentaires le réduit, sauf qu'un commentaire supprimé ayant des réponses est conservé comme espace réservé afin que les réponses restent cohérentes.
- **SSO Users** : le nombre total d'utilisateurs SSO créés par votre site, y compris les personnes ayant commenté une seule fois il y a longtemps. Si c'est la plus grande partie de votre facture, demandez au support une facturation par utilisateurs actifs mensuels, où seuls les utilisateurs connectés durant le mois sont comptés.
- **Moderators**, **Admins**, **SSO Moderators** et **SSO Admins** : des sièges, décrits dans [Admin and Moderator Seats](#admin-and-moderator-seats). Le premier administrateur est gratuit.
- **Domains** : les sites de votre compte. Le premier site est gratuit et chacun des suivants est facturé par mois, tant qu'il est présent dans le compte. Un sous-domaine ne compte que si vous l'ajoutez comme site à part entière ; un seul domaine de premier niveau couvre ses sous-domaines. Voir [Multiple Sites](/guide-multiple-sites.html#domain-limits).
- **Managed Tenants** : les sous-comptes gérés par ce compte, le cas échéant, plus leur propre utilisation sous forme de lignes distinctes.

### Minimum

Si les lignes totalisent moins que le minimum Flex, la facture est complétée jusqu'au minimum. Un petit site avec un administrateur et un domaine ne paie que le minimum.

### Prévisualisation de la facture

La page [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) affiche chaque ligne de la prochaine facture avec les chiffres du jour, pour éviter les surprises. Les compteurs de pages vues, d'API et de petits widgets peuvent avoir jusqu'à une minute de retard ; le reste est en direct.

### Autres devises

L'utilisation Flex est facturée en dollars américains. Si la devise de votre facture est différente, chaque ligne est convertie au taux de change du jour de la création de la facture, de sorte que la même utilisation peut coûter des montants légèrement différents d'un mois à l'autre. Voir [Currencies](#currencies).