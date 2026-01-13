Pour le développement local, utilisez un outil comme [ngrok](https://ngrok.com/).

Afin de simplifier le maintien de la sécurité du système, le développement local suit le même processus que la configuration et la sécurisation des autres environnements. 

### Étape 1 : Ajoutez "localhost" aux domaines de votre compte.

Ajoutez "localhost" [en tant que domaine ici](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Étape 2 : Choisissez une clé API

Nous allons ajouter une configuration de webhook pour votre domaine, donc nous aurons besoin d'une clé API. [Vous pouvez le faire ici.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Sous « Associate with domain » - sélectionnez votre domaine "localhost".

**REMARQUE : Alternativement, vous pouvez utiliser un seul API Secret pour toute l'activité de test et les environnements de préproduction. Ajoutez simplement un API Secret pour "All Domains", et donnez-lui un nom comme "test".**

Assurez-vous d'avoir un API Secret défini pour vos domaines de production. Les événements pour tous les autres domaines utiliseront le secret générique (de test).

### Étape 3 : Ajoutez votre webhook

Pendant que ngrok ou un outil similaire est en cours d'exécution, définissez la valeur pour "localhost" [ici](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

Lorsque vous cliquez sur `Send Test Payload`, nous enverrons deux événements de test pour vérifier que vous validez la clé API.

Une fois validé, cliquez sur `Save`.

### Étape 4 : Ajoutez un commentaire

Vous pouvez maintenant ajouter, modifier ou supprimer des commentaires et devriez nous voir appeler votre machine de développement locale avec les événements, en utilisant votre clé API de test. Il peut y avoir un délai allant jusqu'à 30 secondes pour que les événements atteignent votre machine.