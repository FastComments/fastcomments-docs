FastComments obsługuje codzienne, cotygodniowe lub comiesięczne podsumowanie e-mailowe dla Moderatorów i Administratorów.

Częstotliwość można skonfigurować <a href="" target="_blank">tutaj</a>.

[app-screenshot-start url='/auth/my-account/edit-notifications?demoDigestFrequencyValue=0'; linkUrl='/auth/my-account/edit-notifications'; selector = '.content form'; alt='Formularz edycji powiadomień, w którym ustawiono codzienne, cotygodniowe lub comiesięczne otrzymywanie podsumowania'; title='Konfiguracja częstotliwości podsumowania' app-screenshot-end]

Oprócz ogólnych statystyk dotyczących komentarzy, zawiera on również listę trzech najnowszych komentarzy wymagających moderacji.

Dla każdego z tych komentarzy dostępne są bezpośrednie linki magiczne do:
- Zatwierdzenia komentarza.
- Oznaczenia komentarza jako przejrzanego i przejścia do strony odpowiedzi.
- Oznaczenia komentarza jako spam.

Te linki dla każdego komentarza automatycznie Cię uwierzytelniają i wykonują odpowiednią akcję bezpośrednio z Twojego e-maila.

Dodatkowo, w podsumowaniu znajduje się przycisk Moderate Comments, który wykonuje to samo uwierzytelnienie i przenosi Cię
na stronę moderacji komentarzy.

Należy pamiętać, że te linki magiczne wygasają po pewnym czasie.

[app-screenshot-start url='/test-e2e/email/tenant-comment-digest?HOST=http%3A%2F%2Flocalhost%3A3001&stats=%7B"hasHistory"%3Atrue%2C"newCommentsCount"%3A10002%2C"hasNewCommentsIncreased"%3Atrue%2C"hasNewCommentsDecreased"%3Afalse%2C"approvedCommentsCount"%3A44%2C"hasApprovedCommentsIncreased"%3Afalse%2C"hasApprovedCommentsDecreased"%3Atrue%2C"spamCommentsCount"%3A21%2C"hasSpamCommentsIncreased"%3Afalse%2C"hasSpamCommentsDecreased"%3Atrue%2C"newUsersCount"%3A30%2C"hasNewUsersIncreased"%3Atrue%2C"hasNewUsersFalse"%3Afalse%7D&BANNER_TEXT=FastComments%20Monthly%20Digest&commentCount=100000&hasCommentsNeedsReview=true&comments=%5B%7B"commenterName"%3A"Devon%20Winrick"%2C"commentHTML"%3A"This%20is%20a%20very%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Devon"%2C"commentHTML"%3A"This%20is%20a%20somewhat%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Bob"%2C"commentHTML"%3A"This%20is%20a%20kind%20of%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Ffastcomments.com%2Fimages%2Funknown-person.png"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%5D&locale=en_us&digestEmail=%7B"tenantId"%3A"tenant-id"%2C"userId"%3A"user-id"%2C"_id"%3A"some-id"%2C"temporaryId"%3A"temporary-id"%7D&API_KEY=T0ph%20123!&rawTemporaryId=xyz'; linkUrl=false; selector = '.content'; alt='Comiesięczny e-mail z podsumowaniem zawierający statystyki komentarzy i trzy komentarze wymagające moderacji, każdy z linkami do zatwierdzenia, odpowiedzi i oznaczenia jako spam'; title='E-mail z podsumowaniem' app-screenshot-end]

#### Typy powiadomień

FastComments wysyła wiele rodzajów e-maili do Moderatorów i Administratorów. W razie potrzeby można zrezygnować z powiadomień `Comment Reply`, jednocześnie
nadal otrzymując powiadomienia `New Comment`, wybierając odpowiednie opcje na stronie `Edit Notifications` pokazanej powyżej.