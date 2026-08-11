Kiedy użytkownik zostawia komentarz lub głos, a nie jest zalogowany lub jego konto jest niezweryfikowane, otrzyma e‑mail z prośbą o potwierdzenie tej akcji.

Staramy się jednak nie zasypywać Twoich użytkowników e‑mailami i nie wyślemy więcej niż jednego e‑maila weryfikacyjnego na sesję. Zobacz sekcję Sesje, aby uzyskać więcej szczegółów.

Domyślnie e‑maile weryfikacji komentarzy wyglądają następująco:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Domyślna treść e‑maila weryfikacyjnego cytująca komentarz Alexandera z przyciskiem potwierdzającym post'; title='E‑mail weryfikacji komentarza' app-screenshot-end]

Domyślnie e‑maile weryfikacji głosów wyglądają następująco:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Domyślny e‑mail proszący Devona o potwierdzenie głosu, wyświetlający skomentowany komentarz oraz przycisk potwierdzenia'; title='E‑mail weryfikacji głosu' app-screenshot-end]

Domyślnie FastComments wyświetli swoje logo i nazwę w stopce tych e‑maili:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='Dolna część e‑maila weryfikacyjnego pokazująca domyślne logo i nazwę FastComments w stopce'; title='Stopka e‑maila' app-screenshot-end]

Jeśli korzystasz z planów Flex lub Pro, [nazwę nadawcy, e‑mail i branding można dostosować](/guide-multiple-sites.html#from-name-email-logo).