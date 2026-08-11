FastComments obsługuje codzienne, tygodniowe lub miesięczne podsumowanie e‑mailowe dla moderatorów i administratorów.

Częstotliwość można skonfigurować <a href="" target="_blank">tutaj</a>.

[app-screenshot-start url='/auth/my-account/edit-notifications?demoDigestFrequencyValue=0'; linkUrl='/auth/my-account/edit-notifications'; selector = '.content form'; alt='Formularz Edytuj Powiadomienia, w którym ustawiono częstotliwość podsumowania na codzienną, tygodniową lub miesięczną'; title='Konfigurowanie częstotliwości podsumowania' app-screenshot-end]

Podczas gdy w podsumowaniu znajdują się ogólne statystyki dotyczące Twoich komentarzy, zostaną również wymienione trzy najnowsze komentarze wymagające przeglądu.

Dla każdego z tych komentarzy podane są bezpośrednie magiczne linki do:
- Zatwierdzenia komentarza.
- Oznaczenia komentarza jako przeglądniętego i przejścia do strony odpowiedzi.
- Oznaczenia komentarza jako spam.

Te linki dla każdego komentarza automatycznie uwierzytelnią Cię i wykonają akcję z Twojego e‑maila.

Dodatkowo w podsumowaniu znajduje się przycisk „Moderuj komentarze”, który wykona to samo uwierzytelnienie i przeniesie Cię do
strony moderacji komentarzy.

Należy pamiętać, że te magiczne linki wygasają po pewnym czasie.

[app-screenshot-start url='/test-e2e/email/tenant-comment-digest?HOST=http%3A%2F%2Flocalhost%3A3001&stats=%7B"hasHistory"%3Atrue%2C"newCommentsCount"%3A10002%2C"hasNewCommentsIncreased"%3Atrue%2C"hasNewCommentsDecreased"%3Afalse%2C"approvedCommentsCount"%3A44%2C"hasApprovedCommentsIncreased"%3Afalse%2C"hasApprovedCommentsDecreased"%3Atrue%2C"spamCommentsCount"%3A21%2C"hasSpamCommentsIncreased"%3Afalse%2C"hasSpamCommentsDecreased"%3Atrue%2C"newUsersCount"%3A30%2C"hasNewUsersIncreased"%3Atrue%2C"hasNewUsersFalse"%3Afalse%7D&BANNER_TEXT=FastComments%20Monthly%20Digest&commentCount=100000&hasCommentsNeedsReview=true&comments=%5B%7B"commenterName"%3A"Devon%20Winrick"%2C"commentHTML"%3A"This%20is%20a%20very%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Devon"%2C"commentHTML"%3A"This%20is%20a%20somewhat%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Bob"%2C"commentHTML"%3A"This%20is%20a%20kind%20of%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Ffastcomments.com%2Fimages%2Funknown-person.png"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%5D&locale=en_us&digestEmail=%7B"tenantId"%3A"tenant-id"%2C"userId"%3A"user-id"%2C"_id"%3A"some-id"%2C"temporaryId"%3A"temporary-id"%7D&API_KEY=T0ph%20123!&rawTemporaryId=xyz'; linkUrl=false; selector = '.content'; alt='Miesięczny e-mail podsumowujący z statystykami komentarzy i trzema komentarzami wymagającymi przeglądu, każdy z linkami zatwierdź, odpowiedz i spam'; title='E-mail podsumowania' app-screenshot-end]

#### Notification Types

FastComments wysyła wiele typów e‑maili do moderatorów i administratorów. W razie potrzeby można zrezygnować z powiadomień o `Comment Reply`, jednocześnie nadal otrzymując powiadomienia o `New Comment`, wybierając odpowiednie opcje na stronie `Edit Notifications` pokazanej powyżej.

---