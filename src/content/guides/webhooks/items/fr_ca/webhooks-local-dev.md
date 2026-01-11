Pour le développement local, utilisez un outil comme [ngrok](https://ngrok.com/).

Afin de simplifier la sécurité du système, le développement local suit le même processus que pour la configuration et la sécurisation des autres environnements. 

### Étape 1 : Ajoutez "localhost" aux domaines de votre compte.

Ajoutez "localhost" [comme domaine ici](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Étape 2 : Choisissez une clé API

Nous allons ajouter une configuration de webhook pour votre domaine, donc nous aurons besoin d'une clé API. [Vous pouvez le faire ici.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Sous "Associate with domain" - sélectionnez votre "localhost" domain.

**REMARQUE : Alternativement, vous pouvez utiliser un seul Secret API pour toute l'activité de test et les environnements de staging. Ajoutez simplement un Secret API pour "All Domains", et donnez-lui un nom comme "test".**

Assurez-vous d'avoir un Secret API défini pour votre(s) domaine(s) de production. Les événements pour tous les autres domaines utiliseront le secret wildcard (de test).

### Étape 3 : Ajoutez votre webhook

Pendant que ngrok ou un outil similaire est en cours d'exécution, définissez la valeur pour "localhost" [ici](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

Once it validates, hit `Save`.

### Étape 4 : Ajoutez un commentaire

Now you can add, edit, or delete comments and should see us call your local development machine with the events, using your testing API key. Il peut y avoir un délai allant jusqu'à 30 secondes
pour que les événements atteignent votre machine.