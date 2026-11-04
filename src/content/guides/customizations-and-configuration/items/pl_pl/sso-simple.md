[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Dzięki Simple SSO możemy przekazać widgetowi komentarzy informacje o użytkowniku, dzięki czemu nie będzie musiał podawać swojej nazwy użytkownika ani adresu e-mail, aby komentować.

Simple SSO możemy skonfigurować w następujący sposób:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

Użytkownik zostanie zalogowany, a w tle zostanie utworzony użytkownik SSO. Jeśli użytkownik zostanie pobrany przez API, będzie miał `createdFromSimpleSSO` ustawione na `true`.

Uwagi:

- Adres e-mail jest unikalnym identyfikatorem dla Simple SSO.
- Podanie adresu e-mail w Simple SSO nie jest wymagane, jednak domyślnie komentarze użytkownika będą wyświetlane jako "Niezweryfikowane". <b>Jeśli nie podano adresu e-mail, użytkownik nie może zostać w pełni uwierzytelniony.</b>
- **NOWOŚĆ** Od stycznia 2022: nazwy użytkowników nie muszą być unikalne w całym fastcomments.com
- Simple SSO może automatycznie tworzyć i aktualizować użytkowników SSO, jeśli podano adres e-mail, a użytkownik nie został pierwotnie utworzony przez Secure SSO. Jeśli payload Simple SSO wskazuje użytkownika utworzonego przez Secure SSO lub API, ten użytkownik pozostaje nietknięty, a odwiedzający otrzymuje osobnego użytkownika.
- Możesz określić odznaki dla użytkownika za pomocą właściwości `badgeConfig`. Tablica `badgeIds` zawiera identyfikatory globalnych odznak do przypisania użytkownikowi. Tablica `pageBadgeIds` zawiera identyfikatory odznak ograniczonych do bieżącej strony (`urlId`) — te odznaki są wyświetlane tylko na stronie, na której zostały przypisane. Jeśli `override` jest ustawione na `true`, zastąpi istniejące wyświetlane odznaki (globalne i ograniczone do strony są nadpisywane niezależnie); jeśli `false`, odznaki zostaną dodane do istniejących.

#### Ustawienia Simple SSO

Simple SSO jest zawsze dostępne. Dwa ustawienia kontrolują, jak bardzo FastComments mu ufa. Możesz je znaleźć, wyszukując SSO w <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">regule dostosowywania</a>. Oba są domyślnie wyłączone i obowiązują niezależnie od tego, czy opcja "Require SSO To View Comments" jest włączona.

Konta, które już używały Simple SSO przed dodaniem tych ustawień, mają oba włączone, więc nic się dla nich nie zmieniło.

Stosuje się tylko reguła dostosowywania najlepiej dopasowana do danej strony, więc jeśli masz więcej niż jedną regułę, ustaw je w każdej z nich. Nowa reguła zaczyna z nimi włączonymi, jeśli którakolwiek z Twoich innych reguł ma je włączone.

##### Automatyczna weryfikacja komentarzy przez Simple SSO

Gdy to ustawienie jest włączone, komentarze od użytkowników Simple SSO są od razu weryfikowane.

Gdy jest wyłączone, komentarze od użytkowników Simple SSO wyświetlają się jako "Niezweryfikowane", dopóki użytkownik nie kliknie linku, który wysyłamy e-mailem na adres z payloadu Simple SSO. Jeśli automatycznie zatwierdzasz tylko zweryfikowane komentarze, one również czekają na ten link. Jeśli w payloadzie nie ma adresu e-mail, komentarz pozostaje niezweryfikowany.

To ustawienie dotyczy tylko komentarzy. Głosy od użytkowników Simple SSO są zawsze liczone od razu.

##### Zezwalaj na administratorów Simple SSO

Gdy to ustawienie jest włączone, użytkownik Simple SSO może być administratorem lub moderatorem Twojego konta:

- Użytkownik Simple SSO, który ma uprawnienia administratora lub moderatora, je zachowuje.
- Użytkownik Simple SSO z tym samym adresem e-mail co jeden z Twoich administratorów lub moderatorów przejmuje ich uprawnienia.

Gdy to ustawienie jest wyłączone, Simple SSO nie nadaje tych uprawnień. Payload Simple SSO wskazujący użytkownika, który je posiada, otrzymuje osobnego użytkownika bez uprawnień, a oryginalny użytkownik pozostaje nietknięty.

Payloady Simple SSO nie są podpisane, więc gdy to ustawienie jest włączone, każdy, kto zna adres e-mail administratora lub jego identyfikator użytkownika, może działać w jego imieniu. Jeśli ma to znaczenie dla Twojej witryny, pozostaw to ustawienie wyłączone i użyj Secure SSO dla swoich administratorów.

Użytkowników utworzonych przez Secure SSO lub API nie można osiągnąć przez Simple SSO, niezależnie od tego, czy to ustawienie jest włączone.