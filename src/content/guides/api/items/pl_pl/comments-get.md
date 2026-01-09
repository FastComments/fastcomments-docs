[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

To API służy do pobierania komentarzy do wyświetlenia użytkownikowi. Na przykład automatycznie filtruje
komentarze niezatwierdzone lub oznaczone jako spam.

### Paginacja

Paginację można wykonać na jeden z dwóch sposobów, w zależności od wymagań dotyczących wydajności i przypadku użycia:

1. Najszybszy: **Prekalkulowana paginacja**:
   1. Tak działa FastComments, gdy korzystasz z naszych wbudowanych widgetów i klientów.
   2. Kliknięcie „dalej” po prostu zwiększa numer strony.
   3. Można to sobie wyobrazić jako pobierane z magazynu klucz-wartość.
   4. W ten sposób wystarczy zdefiniować parametr `page` zaczynający się od `0` oraz kierunek sortowania `direction`.
   5. Rozmiary stron można dostosować za pomocą reguł dostosowywania.
2. Najbardziej elastyczny: **Elastyczna paginacja**:
   1. W ten sposób możesz zdefiniować niestandardowe parametry `limit` i `skip`. Nie przekazuj `page`.
   2. Obsługiwany jest również kierunek sortowania `direction`.
   3. `limit` to łączna liczba do zwrócenia po zastosowaniu `skip`.
      - Przykład: ustaw `skip = 200, limit = 100` gdy `page size = 100` i `page = 2`.
   4. Komentarze podrzędne nadal liczą się w paginacji. Można to obejść, używając opcji `asTree`.
      - Możesz paginować dzieci za pomocą `limitChildren` i `skipChildren`.
      - Możesz ograniczyć głębokość zwracanych wątków za pomocą `maxTreeDepth`.

### Wątki

1. Przy użyciu `Precalculated Pagination`, komentarze są grupowane według *strony*, a komentarze w wątkach wpływają na ogólną stronę.
   1. W ten sposób wątki można określić po stronie klienta na podstawie `parentId`.
   2. Na przykład, mając stronę z jednym komentarzem najwyższego poziomu i 29 odpowiedziami, przy ustawieniu `page=0` w API - otrzymasz tylko komentarz najwyższego poziomu i 29 dzieci.
   3. [Example image here illustrating multiple pages.](https://blog.winricklabs.com/images/fc-pagination02.png)
2. Przy użyciu `Flexible Pagination`, możesz zdefiniować parametr `parentId`.
   1. Ustaw go na null, aby pobrać tylko komentarze najwyższego poziomu.
   2. Aby wyświetlić wątki, wywołaj API ponownie i przekaż `parentId`.
   3. Popularnym rozwiązaniem jest wykonanie wywołania API dla komentarzy najwyższego poziomu, a następnie równoległe wywołania API, aby pobrać komentarze dla dzieci każdego komentarza.
3. __NOWE Od lutego 2023!__ Pobierz jako drzewo używając `&asTree=true`.
   1. Można to traktować jako `Flexible Pagination as a Tree`.
   2. Tylko komentarze najwyższego poziomu liczą się w paginacji.
   3. Ustaw `parentId=null`, aby rozpocząć drzewo od korzenia (musisz ustawić `parentId`).
   4. Ustaw `skip` i `limit` dla paginacji.
   5. Ustaw `asTree` na `true`.
   6. Koszt kredytów wzrasta o `2x`, ponieważ nasz backend musi wykonać znacznie więcej pracy w tym scenariuszu.
   7. Ustaw `maxTreeDepth`, `limitChildren` i `skipChildren` według potrzeb.

### Wyjaśnienie drzew

Gdy używasz `asTree`, trudno przewidzieć paginację. Oto pomocna grafika:

<div class="screenshot white-bg">
    <div class="title">Tree Pagination Diagram</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Tree Pagination Diagram" />
</div>

### Pobieranie komentarzy w kontekście użytkownika

API `/comments` może być używane w dwóch kontekstach, dla różnych przypadków użycia:

- Do zwracania komentarzy posortowanych i oznaczonych informacjami do budowy własnego klienta.
  - W tym przypadku zdefiniuj parametr zapytania `contextUserId`.
- Do pobierania komentarzy z twojego backendu do niestandardowych integracji.
  - Platforma domyślnie będzie używać tego bez `contextUserId`. 

[inline-code-attrs-start title = 'Prekalkulowana paginacja komentarzy'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Elastyczna paginacja komentarzy'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Elastyczna paginacja komentarzy w kontekście użytkownika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Elastyczna paginacja komentarzy w kontekście użytkownika — tylko komentarze najwyższego poziomu'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Pobierz komentarze jako drzewo

Możliwe jest pobranie komentarzy zwróconych jako drzewo, gdzie paginacja liczy tylko komentarze najwyższego poziomu.

[inline-code-attrs-start title = 'Komentarze jako drzewo w kontekście użytkownika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Chcesz pobrać tylko komentarze najwyższego poziomu i bezpośrednie dzieci? Oto jeden sposób:

[inline-code-attrs-start title = 'Komentarze jako drzewo z maksymalną głębokością'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Jednak w interfejsie użytkownika możesz potrzebować wiedzieć, czy pokazać przycisk „show replies” przy każdym komentarzu. Podczas pobierania komentarzy jako drzewo do komentarzy, gdy dotyczy, dołączana jest właściwość `hasChildren`.

### Pobierz komentarze jako drzewo, wyszukując według hashtagu

Możliwe jest wyszukiwanie po hashtagu za pomocą API, w całym tenantcie (nie ograniczając się do jednej strony ani `urlId`).

W tym przykładzie pomijamy `urlId` i wyszukujemy po wielu hashtagach. API zwróci tylko komentarze, które zawierają wszystkie żądane hashtagi.

[inline-code-attrs-start title = 'Komentarze jako drzewo w kontekście użytkownika, według hashtagu'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### Wszystkie parametry żądania

[inline-code-attrs-start title = 'Struktura żądania komentarzy'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Identyfikator urlId (adres strony lub identyfikator artykułu), z którym powiązane są komentarze. **/
    urlId?: string
    /** Ogranicz komentarze zwracane przez tego użytkownika. **/
    userId?: string
    /** Użyj tego do wyszukiwania po hashtagu. Aby zawęzić do przecięcia wielu hashtagów, użyj &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** Kierunek sortowania. Domyślnie MR (Most Relevant). Pozostałe opcje to OF (Oldest First) i NF (Newest First). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Prekalkulowana paginacja: Strona do pobrania, zaczynając od 0. Przekaż -1, aby pobrać wszystkie komentarze (do 250). **/
    page?: number
    /** Elastyczna paginacja: Ile komentarzy powinniśmy zwrócić? **/
    limit?: number
    /** Elastyczna paginacja: Ile komentarzy podrzędnych powinniśmy zwrócić dla każdego rodzica? **/
    limitChildren?: number
    /** Elastyczna paginacja: Ile komentarzy powinniśmy pominąć? **/
    skip?: number
    /** Elastyczna paginacja: Ile komentarzy podrzędnych powinniśmy pominąć dla każdego rodzica? **/
    skipChildren?: number
    /** Do określania zablokowanych i zgłoszonych komentarzy. **/
    contextUserId?: string
    /** Do określania zablokowanych i zgłoszonych komentarzy. **/
    anonUserId?: string
    /** Do pobierania komentarzy podrzędnych. **/
    parentId?: string
    /** Do pobierania jako drzewo. **/
    asTree?: boolean
    /** Jak głęboko w drzewie powinny być zwrócone dane? 0 zwraca brak dzieci. 1 zwraca bezpośrednie dzieci itd. **/
    maxTreeDepth?: number
}
[inline-code-end]

### Odpowiedź

[inline-code-attrs-start title = 'Struktura odpowiedzi komentarzy'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Dołączane w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Dołączane w przypadku niepowodzenia. **/
    reason?: string
    /** Komentarze! **/
    comments: Comment[]
}
[inline-code-end]

### Przydatne wskazówki

#### Identyfikator URL

Prawdopodobnie chcesz użyć API `Comment` z parametrem `urlId`. Możesz najpierw wywołać API `Pages`, aby zobaczyć, jak wyglądają dostępne wartości `urlId`. 

#### Anonimowe działania

Dla anonimowego komentowania prawdopodobnie chcesz przekazać `anonUserId` podczas pobierania komentarzy oraz podczas wykonywania zgłoszeń i blokad.

(!) Jest to wymagane przez wiele sklepów z aplikacjami, ponieważ użytkownicy muszą mieć możliwość zgłaszania treści tworzonych przez użytkowników, które widzą, nawet jeśli nie są zalogowani. Brak takiej możliwości może spowodować usunięcie Twojej aplikacji z danego sklepu.

#### Komentarze nie są zwracane

Sprawdź, czy Twoje komentarze zostały zatwierdzone i nie są spamem.

---