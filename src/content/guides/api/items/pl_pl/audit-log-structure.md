An `AuditLog` jest obiektem, który reprezentuje zdarzenie audytowane dla najemców, którzy mają dostęp do tej funkcji.

Struktura obiektu AuditLog jest następująca:

[inline-code-attrs-start title = 'Struktura AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string;
    /** Kto wykonał zdarzenie. **/
    userId?: string;
    username?: string;
    resourceName: string;
    crudType: 'c' | 'r' | 'u' | 'd' | 'login';
    from: string;
    url?: string;
    ip?: string;
    /** Przeglądarka, która wykonała zdarzenie, jeśli pochodziło ono z przeglądarki. **/
    ua?: string;
    /** Hash sesji, z której pochodzi zdarzenie, służący do powiązania działań jednej osoby. Nigdy nie samej sesji. **/
    sIdHashed?: string;
    when: string;
    description?: string;
    serverStartDate: string;
    /** Identyfikator obiektu, na którym wykonano zdarzenie, w przeciwieństwie do tego, kto je wykonał. **/
    targetId?: string;
    /** Czytelna etykieta tego obiektu, np. "jsmith (jsmith@example.com)". **/
    targetLabel?: string;
    objectDetails?: object;
}
[inline-code-end]

`targetId` i `targetLabel` opisują, na czym zdarzenie zostało wykonane; `userId` i `username` opisują, kto je wykonał. W przypadku aktualizacji, `objectDetails.changes` zawiera mapę `{field: {from, to}}` przedstawiającą, co faktycznie się zmieniło.

Dziennik audytu jest niezmienny. Nie może być również zapisywany ręcznie. FastComments.com może jedynie decydować, kiedy zapisać do dziennika audytu. Jednak możesz odczytywać go za pomocą tego API.

Zdarzenia w dzienniku audytu wygasają po dwóch latach.