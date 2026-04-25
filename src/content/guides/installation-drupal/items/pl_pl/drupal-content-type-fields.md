Dla większości stron najłatwiejszym sposobem dodania komentarzy jest dołączenie pola `FastComments` do swoich typów treści. Przejdź do `Structure > Content types > [type] > Manage fields` i dodaj to pole.

Każda encja, która ma to pole, otrzymuje:

- A **status toggle** tak, aby redaktorzy mogli włączać lub wyłączać komentowanie dla poszczególnych encji.
- Opcjonalny **custom identifier**, dzięki któremu możesz używać stabilnego ID, które nie jest powiązane ze ścieżką encji Drupala.

Główny blok `FastComments Widget` rozpoznaje to pole i pominie encje, które mają je już dołączone. Dzięki temu możesz mieszać komentarze przypisane do poszczególnych encji z blokiem bez podwójnego wyświetlania widgetu na tej samej stronie.