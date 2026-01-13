Hier führen wir Schritt für Schritt durch, wie man die FastComments API aufruft, um die Zugriffskontrolle einzurichten.

Bevor wir beginnen: Es ist nicht erforderlich, eine `Group`-Struktur explizit zu erstellen. Gruppen sind einfach Bezeichner
added to `Users` and `Pages`. Adding a group to a user or page automatically "creates" the group.

Zuerst erstellen wir zwei Benutzer, `User A` und `User B`; wir beginnen damit, sie in `Group X` zu platzieren:

[inline-code-attrs-start title = 'cURL-Beispiel: Benutzer A erstellen'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'cURL-Beispiel: Benutzer B erstellen'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Erstellen wir nun eine `Page`. Wir nennen sie unsere `Confidential Page`; bisher hat keiner dieser Benutzer Zugriff darauf, da sie in der Gruppe `CONFIDENTIAL` sein wird:

[inline-code-attrs-start title = 'cURL-Beispiel: Seite erstellen (POST)'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Benutzer A und B haben derzeit **KEINEN** Zugriff auf die neue Seite. Da sie jedoch in derselben Gruppe `GROUP-X` sind, können sie sich gegenseitig `@mention`en.

Aktualisieren wir `User B`, damit er jetzt Zugriff auf die Seite hat:

[inline-code-attrs-start title = 'cURL-Beispiel: Benutzer B aktualisieren'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B` gehört jetzt beiden Gruppen an. Unsere Benutzer können sich weiterhin gegenseitig `@mention`en, aber nur `User B` kann unsere vertrauliche Seite ansehen.

Stellen wir es so ein, dass `User B` nur die vertrauliche Seite ansehen kann:

[inline-code-attrs-start title = 'cURL-Beispiel: Benutzer B aktualisieren'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Jetzt kann er die vertrauliche Seite ansehen, aber keiner unserer Benutzer kann sich gegenseitig `@mention`en, da sie in unterschiedlichen Gruppen sind.

Jeder Benutzer, der nicht Teil der Zugriffskontrolle ist, **kann jedoch auf unsere Seite zugreifen**. Um dies zu verhindern, stellen Sie sicher, dass keinem SSO-Benutzer `groupIds` auf null gesetzt ist. Erstellen wir zum Beispiel `User C`, der Zugriff auf alles hat:

[inline-code-attrs-start title = 'cURL-Beispiel: Benutzer C erstellen'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Indem wir `groupIds` auf null setzen, geben wir an, dass sie nicht durch Zugriffskontrolle eingeschränkt sind.

Erstellen wir nun eine Seite, auf die jeder Zugriff hat:

[inline-code-attrs-start title = 'cURL-Beispiel: Seite erstellen (POST)'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Indem wir `accessibleByGroupIds` auf null setzen, geben wir an, dass diese `Page` nicht über Zugriffskontrolle gesteuert wird, und beide Benutzer können darauf zugreifen.

Damit ist unsere API-Anleitung zur Zugriffskontrolle abgeschlossen.