FastComments Access Control działa poprzez przypisywanie `Pages` i `Users` do wybranych grup.

Grupa to po prostu identyfikator w postaci ciągu znaków, np. `GREEN` lub `abc-123`.

`Users` i `Pages` nie są ograniczone do jednej grupy. Są ograniczone odpowiednio do `100` i `1000` grup. 

#### Dostęp do nieautoryzowanych stron

Jeśli użytkownik spróbuje uzyskać dostęp do strony, do której nie ma uprawnień, zobaczy komunikat o błędzie, jak poniżej:

<div class="screenshot white-bg">
    <div class="title">Przykład niepowodzenia autoryzacji</div>
    <img class="screenshot-image" src="/images/sso-unauthorized-message.png" alt="Przykład niepowodzenia autoryzacji" />
</div>

Tekst komunikatu można dostosować.