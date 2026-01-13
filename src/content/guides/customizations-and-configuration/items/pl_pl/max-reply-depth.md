[related-parameter-start name = 'maxReplyDepth'; type = 'number'; related-parameter-end]

Domyślnie FastComments pozwala na nieograniczone zagnieżdżanie odpowiedzi, tworząc strukturę wątków, w której użytkownicy mogą odpowiadać na odpowiedzi w nieskończoność.

Opcja maxReplyDepth pozwala ograniczyć, jak głęboko mogą sięgać wątki odpowiedzi. Gdy osiągnięty zostanie maksymalny poziom, użytkownicy nie będą już widzieć przycisku odpowiedzi przy komentarzach na tym poziomie.

[code-example-start config = {maxReplyDepth: 2}; linesToHighlight = [6]; title = 'Limiting Reply Depth to 2 Levels'; code-example-end]

Przy ustawieniu maxReplyDepth na 2:
- Użytkownicy mogą komentować na najwyższym poziomie (głębokość 0)
- Użytkownicy mogą odpowiadać na komentarze najwyższego poziomu (głębokość 1)
- Użytkownicy mogą odpowiadać na te odpowiedzi (głębokość 2)
- Dalsze odpowiedzi poza głębokością 2 nie są dozwolone

Ustawienie na 1 pozwoliłoby tylko na odpowiedzi na komentarze najwyższego poziomu, tworząc płytszą strukturę dyskusji.

Ustawienie maxReplyDepth na 0 wyłączyłoby wszystkie odpowiedzi, pozwalając jedynie na komentarze najwyższego poziomu. Jeśli nie zostanie określone, odpowiedzi mogą być zagnieżdżane bez ograniczeń.