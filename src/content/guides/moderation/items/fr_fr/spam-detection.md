Par défaut, FastComments est livré avec un système de détection du spam entraînable.

Au fur et à mesure que vous modérez les commentaires et que vous les marquez comme **Spam**, ou que vous marquez des commentaires automatiquement identifiés comme **Spam** comme **Not Spam**, le système de détection du spam apprendra de ces actions pour déterminer plus précisément ce que vous considérez comme du spam.

Les commentaires marqués comme **Spam** ne seront pas approuvés automatiquement, ils ne seront donc pas affichés tant qu'ils n'auront pas été explicitement marqués comme **Not Spam**.

La détection du spam peut être désactivée via la page Paramètres de modération des commentaires.

### Différents détecteurs de spam

FastComments prend en charge trois méthodes de détection du spam :

1. Un classifieur bayésien naïf traditionnel qui est continuellement entraîné, partagé entre tous les tenants FastComments.com.
2. Un classifieur bayésien naïf traditionnel qui est continuellement entraîné, et qui est **isolated** à votre tenant.
3. Utilisation de ChatGPT 4.

Tous ont accès aux classifieurs bayésiens naïfs partagés et isolés.

L'option ChatGPT 4 est sélectionnable dans la page Paramètres de modération des commentaires si vous êtes sur Flex billing, car elle est facturée en fonction des tokens utilisés.

### Facteur de confiance

FastComments ajuste le filtre anti-spam pour un utilisateur en fonction du degré de confiance que l'on lui accorde pour le site donné.

Par exemple, si des administrateurs ont mis en évidence de nombreux commentaires d'un utilisateur, alors il est probablement très digne de confiance. Ou, s'il est membre du site depuis longtemps et possède beaucoup de commentaires, son facteur de confiance peut aussi être élevé.

### SSO

Les commentaires publiés par des utilisateurs SSO peuvent être considérés comme du spam et seront vérifiés en conséquence. L'exception est si l'utilisateur SSO a le même e-mail qu'un utilisateur tenant qui possède une ou plusieurs des permissions suivantes :

- Account Owner
- Super Admin
- Comment Moderator Admin

Les utilisateurs SSO disposant de ces permissions ne verront pas leurs commentaires vérifiés pour le spam.

### Messages répétés

FastComments détectera et empêchera les messages répétés. Il détectera également les messages répétés très similaires afin d'aider à prévenir le spam. Cela ne peut pas être désactivé car cela empêche notre plateforme d'être utilisée à des fins abusives. Si vous avez un facteur de confiance élevé, cela est pris en compte lors de la prévention des messages répétés.