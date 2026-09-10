## Dépannage

**"Vous n'avez pas la permission" lors de la connexion.** L'utilisateur connecté n'est pas un administrateur API sur le compte.  
Demandez au propriétaire du compte d'accorder la permission API sur la page Utilisateurs, ou connectez‑vous en tant que propriétaire.

**La connexion est étiquetée avec le mauvais site.** La page de consentement connecte le compte avec lequel vous étiez  
connecté au moment. Déconnectez‑vous dans Zapier, changez de compte dans le tableau de bord FastComments, puis reconnectez‑vous.

**Les événements ont cessé d'arriver.** Vérifiez la page Webhooks dans le tableau de bord. Un abonnement dont le point de terminaison a  
échoué pendant six jours est désactivé automatiquement et indique la raison. Réactivez‑le là‑bas, ou désactivez et réactivez le Zap  
à nouveau. Si l'abonnement est complètement absent, quelqu'un l'a supprimé ; désactiver et réactiver le Zap le recrée.

**Zapier indique que le compte doit être reconnecté.** La connexion a été révoquée depuis la page Applications connectées  
, l'utilisateur qui l'a approuvée a perdu la permission API, ou le compte a été supprimé. Reconnectez‑vous depuis Zapier.

**Une action échoue avec "n'a pas d'accès en écriture".** La connexion a été approuvée avec une permission en lecture seule  
. Reconnectez‑vous et approuvez les deux permissions.

**Limites de débit et crédits.** Les actions et recherches consomment des crédits API de votre forfait et sont soumises aux  
mêmes limites de débit que l'API REST. Les déclencheurs n'en consomment aucun. Un Zap qui atteint une limite est réessayé par Zapier  
après le délai indiqué par FastComments.

**Le menu déroulant Domaine est vide.** Les domaines apparaissent une fois configurés sur la page Domaines du  
tableau de bord FastComments. Laissez le champ vide pour recevoir les événements de tous les domaines.