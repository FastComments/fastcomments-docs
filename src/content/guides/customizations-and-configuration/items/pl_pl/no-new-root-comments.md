---
[related-parameter-start name = 'noNewRootComments'; type = 'boolean'; related-parameter-end]

Ustawienie `noNewRootComments` na `true` spowoduje, że widżet ukryje obszar odpowiedzi głównej, ale nadal pozwoli użytkownikom odpowiadać
na komentarze podrzędne. Możesz na przykład ustawić to warunkowo podczas ładowania strony, aby tylko niektórzy użytkownicy mogli zostawiać komentarze najwyższego poziomu.

[code-example-start config = {noNewRootComments: true}; linesToHighlight = [6]; title = 'Prevent New Root Comments'; code-example-end]

---