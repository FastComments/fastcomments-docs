Poniżej przejdziemy przez wywołania API FastComments, aby skonfigurować kontrolę dostępu.

Zanim zaczniemy, pamiętaj, że nie musimy jawnie tworzyć struktury `Group`. Grupy to po prostu identyfikatory
dodawane do `Users` i `Pages`. Dodanie grupy do użytkownika lub strony automatycznie "tworzy" grupę.

Najpierw utwórzmy dwóch użytkowników, `User A` i `User B`; umieścimy ich początkowo w `Group X`:

[inline-code-attrs-start title = 'Przykład cURL tworzenia użytkownika A'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Przykład cURL tworzenia użytkownika B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Teraz utwórzmy `Page`. Nazwiemy ją `Confidential Page`; póki co żaden z tych użytkowników nie będzie miał do niej dostępu, ponieważ będzie ona w grupie `CONFIDENTIAL`:

[inline-code-attrs-start title = 'Przykład cURL POST tworzenia strony'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Użytkownicy A i B obecnie **NIE MAJĄ** dostępu do nowej strony. Jednak ponieważ należą do tej samej grupy, `GROUP-X`, mogą się wzajemnie `@mention`.

Zaktualizujmy `User B`, aby mógł teraz uzyskać dostęp do strony:

[inline-code-attrs-start title = 'Przykład cURL aktualizacji użytkownika B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B` należy teraz do obu grup. Nasi użytkownicy nadal mogą się wzajemnie `@mention`, ale tylko `User B` może przeglądać naszą poufną stronę.

Ustawmy tak, aby `User B` mógł jedynie przeglądać poufną stronę:

[inline-code-attrs-start title = 'Przykład cURL aktualizacji użytkownika B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Teraz może przeglądać poufną stronę, ale żaden z naszych użytkowników nie może już się wzajemnie `@mention`, ponieważ należą do różnych grup.

Jednak każdy użytkownik, który nie bierze udziału w kontroli dostępu, **będzie mógł uzyskać dostęp do naszej strony**. Aby temu zapobiec, upewnij się, że żaden użytkownik SSO nie ma `groupIds` ustawionego na null. Na przykład, utwórzmy `User C`, który ma dostęp do wszystkiego:

[inline-code-attrs-start title = 'Przykład cURL tworzenia użytkownika C'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Ustawiając `groupIds` na null, oznaczamy, że nie są oni ograniczeni przez kontrolę dostępu.

Teraz utwórzmy stronę, do której wszyscy mają dostęp:

[inline-code-attrs-start title = 'Przykład cURL POST tworzenia strony'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Ustawiając `accessibleByGroupIds` na null, oznaczamy, że ta `Page` nie jest kontrolowana przez mechanizm kontroli dostępu i obaj użytkownicy mogą ją odwiedzać.

To kończy nasze omówienie API dotyczące kontroli dostępu.