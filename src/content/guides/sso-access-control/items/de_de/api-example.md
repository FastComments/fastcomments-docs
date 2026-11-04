Hier führen wir durch den Aufruf der FastComments API zur Einrichtung der Zugriffskontrolle.

Bevor wir beginnen, beachten Sie, dass wir keine `Group`-Struktur explizit erstellen müssen. Gruppen sind einfach Identifikatoren,
die zu `Users` und `Pages` hinzugefügt werden. Das Hinzufügen einer Gruppe zu einem Benutzer oder einer Seite „erstellt" die Gruppe automatisch.

Zuerst erstellen wir zwei Benutzer, `User A` und `User B`, die wir zunächst in `Group X` anlegen:

[inline-code-attrs-start title = 'cURL-Beispiel zum Erstellen von User A'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'cURL-Beispiel zum Erstellen von User B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Jetzt erstellen wir eine `Page`. Wir nennen sie unsere `Confidential Page`, und bisher haben keine dieser Benutzer Zugriff darauf, da sie
in der Gruppe `CONFIDENTIAL` sein wird:

[inline-code-attrs-start title = 'Page POST cURL-Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Die Benutzer A und B haben derzeit **KEINEN** Zugriff auf die neue Seite. Da sie sich jedoch in derselben Gruppe, `GROUP-X`, befinden, können sie sich gegenseitig `@mention`.

Lassen Sie uns `User B` aktualisieren, damit er nun auf die Seite zugreifen kann:

[inline-code-attrs-start title = 'cURL-Beispiel zum Aktualisieren von User B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B` gehört nun beiden Gruppen an. Unsere Benutzer können sich weiterhin gegenseitig `@mention`, aber nur `User B` kann unsere vertrauliche Seite anzeigen.

Lassen Sie uns es so einrichten, dass `User B` nur die vertrauliche Seite anzeigen kann:

[inline-code-attrs-start title = 'cURL-Beispiel zum Aktualisieren von User B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Jetzt kann er die vertrauliche Seite anzeigen, aber keiner unserer Benutzer kann den anderen `@mention`, da sie sich in unterschiedlichen Gruppen befinden.

Allerdings **wird jeder Benutzer, der nicht Teil der Zugriffskontrolle ist, auf unsere Seite zugreifen können**. Um dies zu verhindern, stellen Sie sicher, dass keine SSO-Benutzer
ihre `groupIds` auf null gesetzt haben. Erstellen wir zum Beispiel `User C`, der auf alles zugreifen kann:

[inline-code-attrs-start title = 'cURL-Beispiel zum Erstellen von User C'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Durch das Setzen von `groupIds` auf null geben wir an, dass er nicht durch die Zugriffskontrolle eingeschränkt ist.

Erstellen wir nun eine Seite, auf die jeder Zugriff hat:

[inline-code-attrs-start title = 'Page POST cURL-Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Durch das Setzen von `accessibleByGroupIds` auf null geben wir an, dass diese `Page` nicht durch die Zugriffskontrolle gesteuert wird, und beide Benutzer können darauf zugreifen.

Damit ist unser API-Durchlauf für die Zugriffskontrolle abgeschlossen.