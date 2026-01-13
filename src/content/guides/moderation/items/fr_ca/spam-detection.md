Par défaut, FastComments est livré avec une détection de spam entraînable.

Au fur et à mesure que vous modérez les commentaires et que vous les marquez comme **Spam**, ou que vous marquez comme **Not Spam** des commentaires automatiquement détectés comme **Spam**, le système de détection de spam apprendra de ces actions pour déterminer plus précisément ce que vous considérez comme du spam.

Les commentaires marqués comme **Spam** ne seront pas approuvés automatiquement, ils ne s’afficheront donc pas tant qu’ils n’auront pas été explicitement marqués comme **Not Spam**.

La détection de spam peut être désactivée via la page Paramètres de modération des commentaires.

### Different Spam Detectors

FastComments prend en charge trois méthodes de détection du spam :

1. Un classificateur Naïve-Bayes traditionnel, entraîné en continu, qui est partagé entre tous les tenants de FastComments.com.
2. Un classificateur Naïve-Bayes traditionnel, entraîné en continu, qui est **isolé** sur votre tenant.
3. L’utilisation de ChatGPT 4.

Tout le monde a accès aux classificateurs Naïve-Bayes partagé et isolé.

L’option ChatGPT 4 est sélectionnable dans la page Paramètres de modération des commentaires si vous êtes sur Flex billing, puisqu’elle est facturée en fonction des tokens utilisés.

### Trust Factor

FastComments ajuste le filtre anti-spam pour un utilisateur en fonction de son niveau de confiance sur le site donné.

Par exemple, si des administrateurs ont épinglé beaucoup de leurs commentaires, il est probable qu’il s’agisse d’un utilisateur très digne de confiance. Ou, s’il est membre du site depuis longtemps et possède beaucoup de commentaires, son facteur de confiance peut également être élevé.

### SSO

Les commentaires postés par des utilisateurs SSO peuvent être considérés comme du spam et seront vérifiés comme tels. L’exception s’applique si l’utilisateur SSO possède le même courriel qu’un utilisateur du tenant qui dispose d’une ou plusieurs des permissions suivantes :

- Account Owner
- Super Admin
- Comment Moderator Admin

Les utilisateurs SSO avec ces permissions ne verront pas leurs commentaires vérifiés pour le spam.

### Repeated Messages

FastComments détectera et empêchera les messages répétés. Il détectera également les messages répétés très similaires afin d’aider à prévenir le spam. Cela ne peut pas être désactivé car cela empêche notre plateforme d’être utilisée à des fins d’abus. Si vous avez un facteur de confiance élevé, cela est pris en compte lors de la prévention des messages répétés.