[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

Lista identyfikatorów do użycia w autouzupełnianiu **@mentions**. Przydatne, gdy chcesz zapobiec oznaczaniu użytkowników, którzy nie mają wspólnych grup.

Jeśli zostanie określone, w autouzupełnianiu będą wyświetlani tylko użytkownicy z innych grup po wpisaniu znaku `@`.

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]