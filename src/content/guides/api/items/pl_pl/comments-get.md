[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

To API służy do pobierania komentarzy do wyświetlenia użytkownikowi. Na przykład automatycznie filtruje niezatwierdzone lub spamowe komentarze.

### Pagination

Paginację można wykonać na jeden z dwóch sposobów, w zależności od wymagań wydajnościowych i przypadku użycia:

1. **Najszybsza: Preobliczona Paginacja**:
   1. Tak działa FastComments, gdy używasz naszych wstępnie zbudowanych widżetów i klientów.
   2. Kliknięcie "next" po prostu zwiększa licznik stron.
   3. Można to traktować jako pobierane z magazynu klucz‑wartość.
   4. W ten sposób po prostu określ parametr `page` zaczynający się od `0` oraz kierunek sortowania jako `direction`.
   5. Rozmiary stron można dostosować za pomocą reguł personalizacji.
2. **Najbardziej elastyczna: Elastyczna Paginacja**:
   1. W ten sposób możesz zdefiniować własne parametry `limit` i `skip`. Nie przekazuj `page`.
   2. Kierunek sortowania `direction` jest również obsługiwany.
   3. `limit` to całkowita liczba zwracana po zastosowaniu `skip`.
      - Przykład: ustaw `skip = 200, limit = 100` przy `page size = 100` i `page = 2`.
   4. Komentarze podrzędne nadal liczą się w paginacji. Można to obejść używając opcji `asTree`.
      - Możesz paginować dzieci za pomocą `limitChildren` i `skipChildren`.
      - Możesz ograniczyć głębokość zwracanych wątków za pomocą `maxTreeDepth`.

### Threads

1. Podczas używania `Preobliczonej Paginacji`, komentarze są grupowane według *strony*, a komentarze w wątkach wpływają na całą stronę.
   1. W ten sposób wątki mogą być określane po stronie klienta na podstawie `parentId`.
   2. Na przykład, przy stronie z jednym komentarzem najwyższego poziomu i 29 odpowiedziami, oraz ustawieniu `page=0` w API – otrzymasz tylko komentarz najwyższego poziomu i 29 dzieci.
2. Podczas używania `Elastycznej Paginacji` możesz zdefiniować parametr `parentId`.
   1. Ustaw go na null, aby otrzymać tylko komentarze najwyższego poziomu.
   2. Następnie, aby zobaczyć wątki, wywołaj API ponownie i przekaż `parentId`.
   3. Typowym rozwiązaniem jest wykonanie wywołania API dla komentarzy najwyższego poziomu, a następnie równoległych wywołań API, aby pobrać komentarze dla dzieci każdego komentarza.
3. __NOWOŚĆ od lutego 2023!__ Pobierz jako drzewo używając `&asTree=true`.
   1. Można to traktować jako `Elastyczna Paginacja jako Drzewo`.
   2. Tylko komentarze najwyższego poziomu liczą się w paginacji.
   3. Ustaw `parentId=null`, aby rozpocząć drzewo od korzenia (musisz ustawić `parentId`).
   4. Ustaw `skip` i `limit` dla paginacji.
   5. Ustaw `asTree` na `true`.
   6. Koszt kredytów zwiększa się `2x`, ponieważ nasz backend musi wykonać znacznie więcej pracy w tym scenariuszu.
   7. Ustaw `maxTreeDepth`, `limitChildren` i `skipChildren` według potrzeb.

### Trees Explained

Podczas używania `asTree` może być trudno zrozumieć paginację. Oto przydatna grafika:

<div class="screenshot white-bg">
    <div class="title">Diagram Paginacji Drzewa</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Diagram Paginacji Drzewa" />
</div>

### Fetching Comments in The Context of a User

API `/comments` może być używane w dwóch kontekstach, dla różnych przypadków użycia:

- Aby zwrócić komentarze posortowane i oznaczone informacjami potrzebnymi do budowy własnego klienta.
  - W tym przypadku zdefiniuj parametr zapytania `contextUserId`.
- Aby pobrać komentarze z Twojego backendu dla własnych integracji.
  - Platforma domyślnie użyje tego bez `contextUserId`. 

[inline-code-attrs-start title = 'Komentarze Preobliczona Paginacja'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Komentarze Elastyczna Paginacja'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Komentarze Elastyczna Paginacja w Kontekście Użytkownika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Komentarze Elastyczna Paginacja w Kontekście Użytkownika tylko dla Komentarzy Najwyższego Poziomu'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Get Comments as a Tree

Możliwe jest otrzymanie komentarzy zwróconych jako drzewo, przy czym paginacja liczy tylko komentarze najwyższego poziomu.

[inline-code-attrs-start title = 'Komentarze jako Drzewo w Kontekście Użytkownika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Chcesz otrzymać tylko komentarze najwyższego poziomu i ich bezpośrednie dzieci? Oto jedno rozwiązanie:

[inline-code-attrs-start title = 'Komentarze jako Drzewo z Maksymalną Głębokością'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Jednak w Twoim interfejsie może być konieczne określenie, czy wyświetlić przycisk "pokaż odpowiedzi" przy każdym komentarzu. Przy pobieraniu komentarzy jako drzewa istnieje właściwość `hasChildren` oznaczona na komentarzach, gdy ma to zastosowanie.

### Get Comments as a Tree, Searching by Hash Tag

Możliwe jest wyszukiwanie po hashtagach przy użyciu API, w całym Twoim najemcy (nie ograniczone do jednej strony ani `urlId`).

W tym przykładzie pomijamy `urlId` i wyszukujemy po wielu hashtagach. API zwróci tylko komentarze, które mają wszystkie żądane hashtagi.

[inline-code-attrs-start title = 'Komentarze jako Drzewo w Kontekście Użytkownika, według Hashtagu'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### All Request Params

[inline-code-attrs-start title = 'Struktura Żądania Komentarzy'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** urlId (adres URL strony lub identyfikator artykułu), z którym powiązane są komentarze. **/
    urlId?: string
    /** Ogranicz liczbę komentarzy zwracanych przez tego użytkownika. **/
    userId?: string
    /** Użyj tego, aby wyszukać po hashtagach. Aby uzyskać przecięcie wielu hashtagów, użyj &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** Kierunek sortowania. Domyślnie MR (Najbardziej Istotne). Inne opcje to OF (Najstarsze Najpierw) i NF (Najnowsze Najpierw). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Preobliczona Paginacja: Strona do pobrania, zaczynająca się od 0. Przekaż -1, aby pobrać wszystkie komentarze (do 250). **/
    page?: number
    /** Elastyczna Paginacja: Ile komentarzy powinniśmy zwrócić? **/
    limit?: number
    /** Elastyczna Paginacja: Ile komentarzy podrzędnych zwrócić dla każdego rodzica? **/
    limitChildren?: number
    /** Elastyczna Paginacja: Ile komentarzy pominąć? **/
    skip?: number
    /** Elastyczna Paginacja: Ile komentarzy podrzędnych pominąć dla każdego rodzica? **/
    skipChildren?: number
    /** Do określenia zablokowanych i oznaczonych komentarzy. **/
    contextUserId?: string
    /** Do określenia zablokowanych i oznaczonych komentarzy. **/
    anonUserId?: string
    /** Do pobierania komentarzy podrzędnych. **/
    parentId?: string
    /** Do pobierania jako drzewo. **/
    asTree?: boolean
    /** Jak głęboko w drzewie zwrócić dane? 0 zwraca brak dzieci. 1 zwraca bezpośrednie dzieci, itd. **/
    maxTreeDepth?: number
}
[inline-code-end]

### The Response

[inline-code-attrs-start title = 'Struktura Odpowiedzi Komentarzy'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Zawarte w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Zawarte w przypadku niepowodzenia. **/
    reason?: string
    /** Komentarze! **/
    comments: Comment[]
}
[inline-code-end]

### Helpful Tips

#### URL ID

Prawdopodobnie chcesz używać API `Comment` z parametrem `urlId`. Możesz najpierw wywołać API `Pages`, aby zobaczyć, jak wyglądają dostępne wartości `urlId`.

#### Anonymous Actions

Dla anonimowego komentowania prawdopodobnie chcesz przekazać `anonUserId` przy pobieraniu komentarzy oraz przy flagowaniu i blokowaniu.

(!) Jest to wymagane w wielu sklepach z aplikacjami, ponieważ użytkownicy muszą mieć możliwość oznaczania treści utworzonych przez użytkowników, które widzą, nawet jeśli nie są zalogowani. Brak tego może spowodować usunięcie Twojej aplikacji z takiego sklepu.

#### Comments Not Being Returned

Sprawdź, czy Twoje komentarze są zatwierdzone i nie są spamem.

---