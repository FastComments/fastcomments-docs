[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

To API służy do pobierania komentarzy w celu wyświetlenia ich użytkownikowi. Na przykład automatycznie filtruje
komentarze niezatwierdzone lub oznaczone jako spam.

### Paginacja

Paginację można wykonać na jeden z dwóch sposobów, w zależności od wymagań wydajnościowych i przypadku użycia:

1. Najszybszy: **Precalculated Pagination** (prekalkulowana paginacja):
   1. Tak działa FastComments, gdy korzystasz z naszych gotowych widgetów i klientów.
   2. Kliknięcie „następna" po prostu zwiększa licznik stron.
   3. Można to sobie wyobrazić jako pobieranie z magazynu klucz-wartość.
   4. W ten sposób wystarczy zdefiniować parametr `page` zaczynający się od `0` oraz kierunek sortowania jako `direction`.
   5. Rozmiary stron można dostosować za pomocą reguł customizacji.
2. Najbardziej elastyczny: **Flexible Pagination** (elastyczna paginacja):
   1. W ten sposób można zdefiniować własne parametry `limit` i `skip`. Nie przekazuj `page`.
   2. Kierunek sortowania `direction` jest również obsługiwany.
   3. `limit` to całkowita liczba wyników do zwrócenia po zastosowaniu `skip`.
      - Przykład: ustaw `skip = 200, limit = 100` przy `page size = 100` i `page = 2`.
   4. Komentarze podrzędne nadal liczą się do paginacji. Można to obejść za pomocą opcji `asTree`.
      - Możesz paginować komentarze podrzędne za pomocą `limitChildren` i `skipChildren`.
      - Możesz ograniczyć głębokość zwracanych wątków za pomocą `maxTreeDepth`.

### Wątki

1. Przy użyciu `Precalculated Pagination` komentarze są grupowane według *strony*, a komentarze w wątkach wpływają na całą stronę.
   1. W ten sposób wątki mogą być określane po stronie klienta na podstawie `parentId`.
   2. Na przykład, przy stronie z jednym komentarzem najwyższego poziomu i 29 odpowiedziami oraz ustawieniu `page=0` w API - otrzymasz tylko komentarz najwyższego poziomu i 29 odpowiedzi.
2. Przy użyciu `Flexible Pagination` można zdefiniować parametr `parentId`.
   1. Ustaw go na null, aby pobierać tylko komentarze najwyższego poziomu.
   2. Następnie, aby wyświetlić wątki, wywołaj API ponownie i przekaż `parentId`.
   3. Częstym rozwiązaniem jest wykonanie wywołania API dla komentarzy najwyższego poziomu, a następnie równoległych wywołań API w celu pobrania komentarzy dla dzieci każdego komentarza.
3. __NOWOŚĆ od lutego 2023!__ Pobieranie jako drzewo za pomocą `&asTree=true`.
   1. Można to sobie wyobrazić jako `Flexible Pagination as a Tree`.
   2. Tylko komentarze najwyższego poziomu liczą się do paginacji.
   3. Ustaw `parentId=null`, aby rozpocząć drzewo od korzenia (musisz ustawić `parentId`).
   4. Ustaw `skip` i `limit` dla paginacji.
   5. Ustaw `asTree` na `true`.
   6. Koszt w kredytach rośnie `2x`, ponieważ nasz backend musi w tym scenariuszu wykonać znacznie więcej pracy.
   7. Ustaw `maxTreeDepth`, `limitChildren` i `skipChildren` według potrzeb.

### Drzewa - wyjaśnienie

Przy użyciu `asTree` trudno jest rozumować o paginacji. Oto pomocny diagram:

<div class="screenshot white-bg">
    <div class="title">Diagram paginacji drzewa</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Diagram paginacji drzewa" />
</div>

### Pobieranie komentarzy w kontekście użytkownika

API `/comments` może być używane w dwóch kontekstach, dla różnych przypadków użycia:

- Do zwracania komentarzy posortowanych i oznaczonych informacjami, aby zbudować własnego klienta.
  - W tym przypadku zdefiniuj parametr zapytania `contextUserId`.
- Do pobierania komentarzy z Twojego backendu dla niestandardowych integracji.
  - Platforma domyślnie użyje tego trybu bez `contextUserId`.

[inline-code-attrs-start title = 'Komentarze - Precalculated Pagination'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Komentarze - Flexible Pagination'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Komentarze - Flexible Pagination w kontekście użytkownika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Komentarze - Flexible Pagination w kontekście użytkownika, tylko komentarze najwyższego poziomu'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Pobieranie komentarzy jako drzewa

Możliwe jest pobranie komentarzy jako drzewa, gdzie paginacja liczy tylko komentarze najwyższego poziomu.

[inline-code-attrs-start title = 'Komentarze jako drzewo w kontekście użytkownika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Chcesz pobrać tylko komentarze najwyższego poziomu i ich bezpośrednie odpowiedzi? Oto jeden ze sposobów:

[inline-code-attrs-start title = 'Komentarze jako drzewo z maksymalną głębokością'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Jednak w Twoim interfejsie użytkownika może być potrzebna informacja, czy wyświetlić przycisk „pokaż odpowiedzi" przy
każdym komentarzu. Przy pobieraniu komentarzy jako drzewa istnieje właściwość `hasChildren` dodawana
do komentarzy, gdy ma to zastosowanie.

### Pobieranie komentarzy jako drzewa, wyszukiwanie po hashtagu

Możliwe jest wyszukiwanie po hashtagu za pomocą API, w obrębie całej Twojej tenanta (nie ograniczone do jednej strony ani `urlId`).

W tym przykładzie pomijamy `urlId` i wyszukujemy według wielu hashtagów. API zwróci tylko komentarze, które zawierają wszystkie żądane hashtagi.

[inline-code-attrs-start title = 'Komentarze jako drzewo w kontekście użytkownika, według hashtagu'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### Wszystkie parametry zapytania

[inline-code-attrs-start title = 'Struktura zapytania Comments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The urlId (page url, or article id) the comments are associated with. **/
    urlId?: string
    /** Limit the comments returned by this user. **/
    userId?: string
    /** Use this to search by hashtag. To drill down to the intersection of multiple hashtags, do &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** The sort direction. Default is MR (Most Relevant). Other options are OF (Oldest First) and NF (Newest First). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Precalculated Pagination: The page to fetch, starting with 0. Pass -1 for all comments (up to 250). **/
    page?: number
    /** Flexible Pagination: How many comments should we return? **/
    limit?: number
    /** Flexible Pagination: How many child comments should we return for each parent? **/
    limitChildren?: number
    /** Flexible Pagination: How many comments should we skip? **/
    skip?: number
    /** Flexible Pagination: How many child comments should we skip for each parent? **/
    skipChildren?: number
    /** For determining blocked and flagged comments. **/
    contextUserId?: string
    /** For determining blocked and flagged comments. **/
    anonUserId?: string
    /** For fetching child comments. **/
    parentId?: string
    /** For fetching as a tree. **/
    asTree?: boolean
    /** How far into the tree should we return data? 0 returns no children. 1 returns immediate children, etc. **/
    maxTreeDepth?: number
}
[inline-code-end]

### Odpowiedź

[inline-code-attrs-start title = 'Struktura odpowiedzi Comments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Included on failure. **/
    reason?: string
    /** The comments! **/
    comments: Comment[]
}
[inline-code-end]

### Przydatne wskazówki

#### URL ID

Prawdopodobnie chcesz używać API `Comment` z parametrem `urlId`. Możesz najpierw wywołać API `Pages`, aby zobaczyć, jak wyglądają dostępne dla Ciebie wartości `urlId`.

#### Akcje anonimowe

Dla anonimowego komentowania prawdopodobnie chcesz przekazywać `anonUserId` przy pobieraniu komentarzy oraz przy zgłaszaniu i blokowaniu.

(!) Jest to wymagane przez wiele sklepów z aplikacjami, ponieważ użytkownicy muszą mieć możliwość zgłaszania treści tworzonych przez użytkowników, które widzą, nawet jeśli nie są zalogowani. Niezastosowanie się może spowodować usunięcie Twojej aplikacji z danego sklepu.

#### Komentarze nie są zwracane

Sprawdź, czy Twoje komentarze są zatwierdzone i nie są oznaczone jako spam.