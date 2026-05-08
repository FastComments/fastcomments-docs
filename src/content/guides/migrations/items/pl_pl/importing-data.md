---
Chociaż zespół wsparcia FastComments może pomóc przy migracjach, większość z nich można wykonać i monitorować łatwo bez interwencji personelu wsparcia.

Natywnie obsługujemy importowanie eksportów od następujących dostawców:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (za pomocą wtyczki)
- AnyComment (przez import/eksport WordPress)

Przechodząc [tutaj](https://fastcomments.com/auth/my-account/manage-data/import) możemy przesłać plik zawierający dane do migracji.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### Monitorowanie importów

FastComments używa systemu przetwarzania zadań do obsługi importów i eksportów. Gdy system podejmie Twoje zadanie, będzie ono okresowo raportować status w interfejsie importu lub eksportu.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

Zauważ, że statusy importów i eksportów są widoczne dla wszystkich administratorów na koncie.

Jeśli Twoje zadanie zakończy się niepowodzeniem, nie zostanie ono automatycznie ponownie uruchomione. Import trzeba będzie spróbować ponownie. Jeśli jakikolwiek import lub eksport się nie powiedzie, nasi administratorzy systemu zostaną automatycznie powiadomieni. Jeśli zidentyfikujemy problem, skontaktujemy się z Tobą, aby sprawdzić, czy możemy pomóc.

### Ponowne uruchomienie importu

Podczas niektórych migracji konieczne jest uruchomienie importu wielokrotnie. Na przykład często wykonuje się pierwsze przejście migracji w celach testowych, a następnie uruchamia import ponownie z najnowszymi danymi przed przełączeniem.

Ponowne importowanie tych samych treści **nie spowoduje duplikatów**.

### Bezpieczeństwo danych i wygasanie

Pliki importu nie są w żaden sposób dostępne na zewnątrz, a pliki importu są usuwane z naszego systemu zaraz po zakończeniu importu.

---