Chociaż FastComments Support może pomóc w migracjach, większość z nich można łatwo przeprowadzić i monitorować bez jakiejkolwiek interwencji
personelu wsparcia.

Natywnie obsługujemy importowanie eksportów z następujących dostawców:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- Cusdis
- WordPress (za pomocą wtyczki lub eksportu XML lub CSV)
- AnyComment (za pomocą Importu/Eksportu WordPress)

Przechodząc [tutaj](https://fastcomments.com/auth/my-account/manage-data/import), możemy przesłać plik zawierający dane do migracji.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; alt='Strona importu FastComments z polami wyboru dostawcy i przesyłania pliku eksportu'; title='Formularz strony importu' app-screenshot-end]

### Monitorowanie importów

FastComments używa systemu przetwarzania zadań do przetwarzania importów i eksportów. Gdy system podejmie Twoje zadanie, będzie
okresowo raportować jego status w interfejsie importu lub eksportu.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; alt='Strona importu pokazująca uruchomione zadanie importu oraz status raportowany przez system przetwarzania zadań'; title='Status zadania importu' app-screenshot-end]

Zauważ, że status importów i eksportów jest widoczny dla wszystkich administratorów w ramach konta.

Jeśli Twoje zadanie się nie powiedzie, nie zostanie automatycznie ponownie uruchomione. Import będzie musiał zostać ponownie podjęty. Jeśli jakikolwiek import lub eksport się nie powiedzie,
nasi administratorzy systemu są automatycznie powiadamiani. Jeśli zidentyfikujemy problem, skontaktujemy się z Tobą, aby sprawdzić, czy możemy pomóc.

### Ponowne uruchamianie importu

Podczas niektórych migracji konieczne jest wielokrotne uruchomienie importu. Na przykład często wykonuje się pierwszą migrację
testową, a następnie ponownie uruchamia import z najnowszymi danymi przed przełączeniem.

Ponowne zaimportowanie tej samej zawartości **nie spowoduje utworzenia duplikatów**.

### Bezpieczeństwo danych i wygasanie

Pliki importu nie są w żaden sposób dostępne dla żądań z zewnątrz, a pliki importu są usuwane z naszego systemu natychmiast
po zakończeniu importu.