Qui esamineremo come chiamare l'API di FastComments per configurare il controllo degli accessi.

Prima di iniziare, nota che non dobbiamo creare esplicitamente una struttura `Group`. I gruppi sono semplicemente identificatori aggiunti a `Users` e `Pages`. Aggiungere un gruppo a un utente o a una pagina "crea" automaticamente il gruppo.

Per prima cosa, creiamo due utenti, `User A` e `User B`, e li inseriremo inizialmente in `Group X`:

[inline-code-attrs-start title = 'Esempio cURL per creare l'Utente A'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-a",
	"username": "User A",
	"email": "usera@example.com",
    "groupIds": ["GROUP-X"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Esempio cURL per creare l'Utente B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-b",
	"username": "User B",
	"email": "userb@example.com",
    "groupIds": ["GROUP-X"]
}'
[inline-code-end]

Ora creiamo una `Page`. La chiameremo la nostra `Confidential Page`, e per ora nessuno di questi utenti avrà accesso ad essa poiché sarà
nel gruppo `CONFIDENTIAL`:

[inline-code-attrs-start title = 'Esempio cURL POST per creare Pagina'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Confidential Page",
	"url": "https://mysite.com/confidential",
	"urlId": "https://mysite.com/confidential",
	"accessibleByGroupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Gli utenti A e B attualmente **NON** hanno accesso alla nuova pagina. Tuttavia, poiché sono nello stesso gruppo, `GROUP-X`, possono `@mention`arsi a vicenda.

Aggiorniamo `User B` in modo che ora possa accedere alla pagina:

[inline-code-attrs-start title = 'Esempio cURL per aggiornare l'Utente B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B` ora appartiene a entrambi i gruppi. I nostri utenti possono ancora `@mention`arsi a vicenda, ma solo `User B` può visualizzare la nostra pagina riservata.

Facciamo in modo che solo `User B` possa visualizzare la pagina riservata:

[inline-code-attrs-start title = 'Esempio cURL per aggiornare l'Utente B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Ora può visualizzare la pagina riservata, ma nessuno dei nostri utenti può più `@mention`arsi a vicenda, poiché si trovano in gruppi diversi.

Tuttavia, qualsiasi utente che non fa parte del controllo degli accessi **potrà accedere alla nostra pagina**. Per evitarlo, assicurati che nessun Utente SSO abbia
i propri `groupIds` impostati su null. Ad esempio, creiamo `User C`, che ha accesso a tutto:

[inline-code-attrs-start title = 'Esempio cURL per creare l'Utente C'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-c",
	"username": "User C",
	"email": "userc@example.com",
    "groupIds": null
}'
[inline-code-end]

Impostando `groupIds` su null, dichiariamo che non sono limitati dal controllo degli accessi.

Ora, creiamo una pagina a cui tutti hanno accesso:

[inline-code-attrs-start title = 'Esempio cURL POST per creare Pagina'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Public Page",
	"url": "https://mysite.com/public",
	"urlId": "https://mysite.com/public",
	"accessibleByGroupIds": null
}'
[inline-code-end]

Impostando `accessibleByGroupIds` su null, dichiariamo che questa `Page` non è controllata tramite il controllo degli accessi, e entrambi gli utenti possono accedervi.

Questo completa la nostra panoramica sull'API per il Controllo degli Accessi.