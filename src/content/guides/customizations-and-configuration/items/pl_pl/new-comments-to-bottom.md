[related-parameter-start name = 'newCommentsToBottom'; type = 'boolean'; related-parameter-end]

Domyślnie nowe komentarze na żywo pojawiają się na górze listy komentarzy w momencie ich publikacji w czasie rzeczywistym.

Po włączeniu tej opcji nowe komentarze na żywo będą dodawane zamiast tego na dół listy. Ma to wpływ na sposób wyświetlania komentarzy, gdy są publikowane na żywo podczas przeglądania wątku komentarzy przez użytkowników.

[code-example-start config = {newCommentsToBottom: true}; linesToHighlight = [6]; title = 'New Live Comments to Bottom'; code-example-end]

Po włączeniu tego ustawienia:
- Nowe komentarze na żywo publikowane przez innych użytkowników będą pojawiać się na dole listy komentarzy
- Użytkownicy będą widzieć nowe komentarze pojawiające się poniżej istniejących komentarzy w czasie rzeczywistym
- To wpływa tylko na aktualizacje komentarzy na żywo - nie na początkowe ładowanie strony
- Może to pomóc w zachowaniu ciągłości czytania, gdy użytkownicy śledzą dyskusję

Zwróć uwagę, że to ustawienie wpływa tylko na to, gdzie umieszczane są nowe komentarze na żywo w momencie ich nadejścia w czasie rzeczywistym. Nie ma wpływu na początkowe ustawienie sortowania przy ładowaniu strony.