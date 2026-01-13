Każda instancja widgetu komentarzy jest izolowana. Z tego powodu FastComments natywnie obsługuje więcej niż jedną instancję na stronie lub wiele instancji wskazujących na ten sam wątek czatu.

W przypadku biblioteki VanillaJS na przykład, wystarczy powiązać widget komentarzy z różnymi węzłami DOM. Jeśli chcesz po prostu zaktualizować bieżący wątek na stronie, zobacz [Przełączanie wątków komentarzy bez przeładowania strony](guide-customizations-and-configuration.html#switching-comment-threads);

### Synchronizacja stanu uwierzytelnienia między wieloma instancjami

Przeanalizujmy przykład niestandardowej aplikacji single-page, która jest listą często zadawanych pytań z własnym wątkiem komentarzy.

W tym przypadku mamy wiele instancji FastComments w DOM jednocześnie.

Jest to w porządku, ale stwarza pewne wyzwania dla doświadczenia użytkownika.

Rozważ ten przepływ:

1. Użytkownik odwiedza stronę z listą pytań, każde z własnym widgetem komentarzy.
2. Użytkownik wprowadza swoją nazwę użytkownika i email i zostawia pytanie w jednym z wątków.
3. Widzi kolejny element FAQ, o który ma pytanie.
4. Idzie skomentować ponownie. Czy musi ponownie wprowadzić swój email i nazwę użytkownika?

W tym przypadku FastComments obsługuje synchronizację stanu uwierzytelnienia między instancjami widgetu za Ciebie. W kroku czwartym użytkownik będzie już tymczasowo uwierzytelniony, ponieważ wprowadził swoją nazwę użytkownika i email na tej samej stronie.
