[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Za pomocą Simple SSO możemy dostarczyć widżetowi komentarzy informacje o użytkowniku, aby nie musiał on wprowadzać swojej nazwy użytkownika ani adresu e‑mail, aby skomentować.

Możemy skonfigurować Simple SSO w następujący sposób:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Proste SSO'; code-example-end]

Użytkownik zostanie zalogowany i w tle zostanie utworzony użytkownik SSO. Użytkownik będzie miał `createdFromSimpleSSO` ustawione na `true`, jeśli zostanie pobrany z API.

**Uwagi:** 

- E‑mail jest unikalnym identyfikatorem dla Simple SSO.
- Podanie e‑maila w Simple SSO nie jest wymagane, jednak domyślnie ich komentarze będą wyświetlane jako „Niezweryfikowane”. <b>Jeśli nie podano e‑maila, użytkownik nie może zostać w pełni uwierzytelniony.</b>
- **NOWOŚĆ** Od stycznia 2022: Nazwy użytkowników nie muszą być unikalne w całej domenie fastcomments.com
- Simple SSO może automatycznie tworzyć i aktualizować użytkowników SSO, jeśli podano e‑mail i użytkownik nie został pierwotnie utworzony z Secure SSO. Jeśli ładunek Simple SSO wskazuje użytkownika utworzonego przez Secure SSO lub API, ten użytkownik pozostaje niezmieniony, a odwiedzający otrzymuje osobnego użytkownika.
- Możesz określić odznaki dla użytkownika za pomocą właściwości `badgeConfig`. Tablica `badgeIds` zawiera identyfikatory globalnych odznak, które mają być powiązane z użytkownikiem. Tablica `pageBadgeIds` zawiera identyfikatory odznak ograniczonych do bieżącej strony (`urlId`) — te odznaki są wyświetlane tylko na stronie, na której zostały przydzielone. Jeśli `override` jest ustawione na `true`, zastąpi istniejące wyświetlane odznaki (globalne i ograniczone do strony są nadpisywane niezależnie); jeśli `false`, zostaną dodane do istniejących odznak.

#### Simple SSO Settings

Simple SSO jest zawsze dostępne. Dwa ustawienia kontrolują, jak bardzo FastComments mu ufa. Możesz je znaleźć, wyszukując SSO w <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">regule dostosowywania</a>. Oba są domyślnie wyłączone i mają zastosowanie niezależnie od tego, czy opcja „Wymagaj SSO, aby wyświetlać komentarze” jest włączona.

Konta, które już korzystały z Simple SSO w momencie dodania tych ustawień, mają oba włączone, więc nic się dla nich nie zmieniło.

Obowiązuje tylko reguła dostosowywania, która najlepiej pasuje do strony, więc jeśli masz więcej niż jedną regułę, ustaw je w każdej z nich. Nowa reguła rozpoczyna się z włączonymi ustawieniami, jeśli którakolwiek z Twoich innych reguł ma je włączone.

##### Auto Verify Comments via Simple SSO

Gdy jest włączone, komentarze od użytkowników Simple SSO są weryfikowane od razu.

Gdy jest wyłączone, komentarze od użytkowników Simple SSO wyświetlane są jako „Niezweryfikowane”, dopóki użytkownik nie kliknie linku, który wysyłamy na adres podany w ładunku Simple SSO. Jeśli automatycznie zatwierdzasz tylko zweryfikowane komentarze, również czekają na ten link. Jeśli w ładunku nie ma e‑maila, komentarz pozostaje niezweryfikowany.

To ustawienie wpływa tylko na komentarze. Głosy od użytkowników Simple SSO są zawsze liczone od razu.

##### Allow Simple SSO Admins

Gdy jest włączone, użytkownik Simple SSO może być administratorem lub moderatorem Twojego konta:

- Użytkownik Simple SSO, który ma uprawnienia administratora lub moderatora, zachowuje je.
- Użytkownik Simple SSO z takim samym e‑mailem jak jeden z Twoich administratorów lub moderatorów przejmuje ich uprawnienia.

Gdy jest wyłączone, Simple SSO nie przyznaje tych uprawnień. Ładunek Simple SSO wskazujący użytkownika, który je posiada, otrzymuje osobnego użytkownika bez uprawnień, a pierwotny użytkownik pozostaje niezmieniony.

Ładunki Simple SSO nie są podpisane, więc gdy jest włączone, każdy, kto zna e‑mail administratora lub jego identyfikator użytkownika, może działać w jego imieniu. Jeśli ma to znaczenie dla Twojej witryny, pozostaw to wyłączone i użyj Secure SSO dla swoich administratorów.

Użytkownicy utworzeni przez Secure SSO lub API nie mogą być osiągnięci przez Simple SSO, niezależnie od tego, czy to jest włączone.